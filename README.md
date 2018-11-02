# ClionRustHelloWorld
## Step 01:
[Install CLion Linux ](https://www.jetbrains.com/clion/download/#section=linux)  

``Note: I used CLion but you can any prepered tools to edit source code``
---------------------------------------------------------------------------------------------------------------------------------
## Step 02:
[Install Rust & Toml Plugings into CLion](https://github.com/damithuoc/ClionRustHelloWorld/blob/master/asserts/CLion-Rust-Toml-Pluging.png)

``Note: Just browse with the plugings names' and install plugins``
---------------------------------------------------------------------------------------------------------------------------------
## Step 03:
[Install Rust in linux](https://doc.rust-lang.org/book/2018-edition/ch01-01-installation.html)
Note: I have used following commands to install Rust
```RUSTUP_HOME=/opt/rust
  export RUSTUP_HOME
  CARGO_HOME=/opt/rust
  export CARGO_HOME
  curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
  
```
If you got OpenSSL error when you are tring to install Rust use this command `sudo apt-get install libssl-dev` 

After install success you can see there is an env file and if you cat it you can this `export PATH="/opt/rust/bin:$PATH"` and  
copy and pase this line to your `~/.barshrc` file and save it, after that enter this command `source ~/.bashrc`

[Dir Structure](https://github.com/damithuoc/ClionRustHelloWorld/blob/master/asserts/Rust-DIR-Structure.png)

### Step 04: 
First check Rust installation success status on your platform, use following commands
```which rustup 
which rustc
which cargo
rustup install stable
rustup default stable
```
[Rust Installation outputs](https://github.com/damithuoc/ClionRustHelloWorld/blob/master/asserts/Rust-Install-Success-Opuputs.png)

[Rust Project Creation Screen in CLion](https://github.com/damithuoc/ClionRustHelloWorld/blob/master/asserts/Initilize_rust-pjojec.png)

Note: If you want to create a Rust project in command line you can use `cargo new ${project_name}` example `cargo new learn_rust`

[Rust Project run configurations in CLion](https://github.com/damithuoc/ClionRustHelloWorld/blob/master/asserts/Clion-Rust-Run-Configurations.png)

### Step 05: 
[Build and Run Rust project](https://github.com/damithuoc/ClionRustHelloWorld/blob/master/asserts/Basic%20Cargo%20Coammnds.png)
