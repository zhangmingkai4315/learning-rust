use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main_v1()-> io::Result<()>{
    let mut f = File::open("sample.txt").await?;
    let mut bufferr = [0;10];

    let n = f.read(&mut bufferr[..]).await?;
    println!("{:?}", n);
    Ok(())
}


#[tokio::main]
async fn main_v2()-> io::Result<()>{
    let mut f = File::open("sample.txt").await?;
    let mut bufferr= Vec::new();

    let n = f.read_to_end(&mut bufferr).await?;
    println!("{:?}", bufferr);
    Ok(())
}

#[tokio::main]
async fn main_v3()-> io::Result<()>{
    let mut f = File::create("sample2.txt").await?;
    let n = f.write(b"some bytes write to file").await?;
    println!("{:?}", n);
    Ok(())
}


#[tokio::main]
async fn main()-> io::Result<()>{
    let mut reader: &[u8] = b"hello";
    let mut f = File::create("sample2.txt").await?;
    io::copy(&mut reader, &mut f).await?;
    Ok(())
}

