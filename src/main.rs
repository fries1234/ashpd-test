use ashpd::desktop::trash;
use async_std::task;
use async_std::fs;

fn main() {
    task::block_on(run())
}

async fn run() {
    tracing_subscriber::fmt::init();
    let test = "This is a test file";
    match fs::write("testfile.txt", test).await {
        Ok(_) => {
            match fs::File::open("testfile.txt").await {
                Ok(file) => {
                    match trash::trash_file(&file).await {
                        Ok(_) => {},
                        Err(err) => {
                            tracing::error!("An error occured while trying to trash the test file: {}", err);
                        },
                    }
                },
                Err(err) => {
                    tracing::error!("An error occured while trying to open the test file: {}", err);
                },
            }
        },
        Err(err) => {
            tracing::error!("An error occured while trying to write the test file data: {}", err);
        },
    }
}
