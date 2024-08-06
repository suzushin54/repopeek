# ECR Image Browser

Simple CLI tool written in Rust to list and pull Docker images stored in AWS ECR repositories. 

![execute-image](https://github.com/user-attachments/assets/feb63014-8eb0-49ad-9621-65170136d063 "execute-image")

## Setup

1. **Configure AWS CLI**:
    ```sh
    aws configure
    ```

2. **Clone the repository**:
    ```sh
    git clone https://github.com/suzushin54/ecr-image-browser.git
    cd ecr-image-browser
    ```

3. **Install dependencies**:
    ```sh
    cargo build
    ```
4. **Run the application**:
    ```sh
    cargo run
    ```

## Usage

1. Run the application
```sh
cargo run
```
2. Select an AWS profile
```sh
? Select an AWS profile:  
> sandbox
  default
[↑↓ to move, enter to select, type to filter]
```
3. Chose an ECR Repository
```sh
> Select an AWS profile: default
? Select a repository:  
> nginx
  the-other-repository 
  empty-repository
[↑↓ to move, enter to select, type to filter]
```
4. Select an image to pull
```sh
> Select an AWS profile: default
> Select a repository: nginx
Selected repository nginx
? Select an image to pull:  
  latest        2024-07-13T09:39:25Z    64 MB   sha256:67c5f921c00b76d0d68182656ba144bfd878c92bc1a5c9521a444d89e49bc00c
> alpine        2024-07-13T09:36:41Z    16 MB   sha256:9459849d2fcb182dc760396be39c55459f5ea4899f18cb25433c514ee5ad582c
[↑↓ to move, enter to select, type to filter]
```

