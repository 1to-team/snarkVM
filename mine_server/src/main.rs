use std::{io, env};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use futures::SinkExt;
use rand::thread_rng;
use rayon::{ThreadPoolBuilder, ThreadPool};
use tokio::sync::{mpsc, Mutex};
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use std::error::Error;
use log::{info, error, warn};
use std::{str::FromStr, time::Instant, sync::atomic::AtomicBool};
use crossterm::tty::IsTty;
use tracing_subscriber::filter::EnvFilter;

use snarkvm_dpc::{Network, BlockTemplate, PoSWScheme, testnet2::Testnet2};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    init_logging();
    info!("Started: {:?}", args);
    let address = args[1].to_string();
    info!("Listening on: {}", address);
    let listener = TcpListener::bind(address).await?;
    loop {
        let (socket, client_addr) = listener.accept().await?;
        info!("New client connected: {:?}", client_addr);
        tokio::spawn(process_socket(socket));
    }
}

fn init_logging() {
    std::env::set_var("RUST_LOG", "trace");

    let filter = EnvFilter::from_default_env()
        .add_directive("mio=off".parse().unwrap())
        .add_directive("tokio_util=off".parse().unwrap())
        .add_directive("hyper::proto::h1::conn=off".parse().unwrap())
        .add_directive("hyper::proto::h1::decode=off".parse().unwrap())
        .add_directive("hyper::proto::h1::io=off".parse().unwrap())
        .add_directive("hyper::proto::h1::role=off".parse().unwrap());

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(io::stdout().is_tty())
        .with_target(true)
        .try_init();
}

async fn process_socket(socket: TcpStream) {
    let mut lines = Framed::new(socket, LinesCodec::new());
    let (tx, mut rx) = mpsc::unbounded_channel();
    let tx = Arc::new(tx);
    let state = Arc::new(Mutex::new(Shared::new()));
    let threads_num = match env::var("SNARKOS_THREADS_NUM") {
        Ok(val) => val.parse::<usize>().unwrap(),
        Err(_e) => 8,
    };
    let thread_pool = Arc::new(ThreadPoolBuilder::new()
        .stack_size(8 * 1024 * 1024)
        .num_threads(threads_num)
        .build().unwrap());

    loop {
        let state = Arc::clone(&state);
        let tx = Arc::clone(&tx);

        info!("Select...");
        tokio::select! {
            result = lines.next() => {
                info!("lines.next(): {:?}", result);
                match result {
                    Some(Ok(msg)) => {
                        if msg == "" {
                            continue;
                        }
                        match msg.chars().next().unwrap() {
                            '{' => process_template(state, tx, thread_pool.clone(), msg).await,
                            'S' => process_stop(state).await,
                            unk => {
                                error!("Unknown command received: {}", unk);
                                break;
                            },
                        }
                    }
                    Some(Err(e)) => {
                        error!("an error occurred while processing messages; error = {:?}", e);
                    }
                    None => break,
                }
            },
            Some(msg) = rx.recv() => {
                info!("rx.recv(): {}", msg);
                lines.send(&msg).await.unwrap();
            },
        }
    }
}

struct Shared {
    pub terminator: Option<Arc<AtomicBool>>,
    pub last_task: Option<JoinHandle<()>>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            terminator: None,
            last_task: None,
        }
    }
}


async fn process_template(state: Arc<Mutex<Shared>>, tx: Arc<mpsc::UnboundedSender<String>>, thread_pool: Arc<ThreadPool>, block_template_str: String) {
    let terminator = Arc::new(AtomicBool::new(false));
    let mut state_locked = state.lock().await;
    state_locked.terminator = Some(terminator.clone());
    state_locked.last_task = Some(tokio::spawn(async move {
        info!("Received block_template: {}", block_template_str);
        let mut block_template = BlockTemplate::<Testnet2>::from_str(&block_template_str).unwrap();
        // TODO: process invalid blocks

        let start = Instant::now();
        info!("Mine start (height = {})", block_template.block_height());
        let (mine_result, iteration) = tokio::task::spawn_blocking(move || {
            let mut iteration = 0;
            thread_pool.install(move || {
                let r = Testnet2::posw().mine2(&mut block_template, &terminator, &mut thread_rng(), &mut iteration);
                (r, iteration)
            })
        })
        .await.unwrap();
        info!("Mine end: {:.2} s (with {} iterations)", (start.elapsed().as_millis() as f64) / 1000.0, iteration);

        let send_str = match mine_result {
            Ok(block_header) => {
                let block_str = block_header.to_string();
                warn!("Mine done: {}", block_str);
                format!("{}|DONE: {}", iteration, block_str)
            },
            Err(error) => {
                warn!("Mine stopped: {:?}", error);
                format!("{}|ERROR: {}", iteration, error)
            }
        };
        tx.send(send_str).unwrap();
        // TODO: process disconnected client
    }));

}

async fn process_stop(state: Arc<Mutex<Shared>>) {
    info!("Received stop command");
    let mut state = state.lock().await;
    if !state.last_task.is_some() {
        error!("not started yet?");
        return;
    }
    state.terminator.as_ref().unwrap().store(true, Ordering::SeqCst);
    state.last_task.as_mut().unwrap().await;
    state.last_task = None;
}