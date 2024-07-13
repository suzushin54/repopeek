# ECR Image Browser

Simple CLI tool written in Rust to list Docker images stored in AWS ECR repositories. 

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


## Optional: Push Docker Images to ECR (if you don't have images in ECR)

1. **Create and configure a `.env` file**:
   Copy the `.env.sample` file to `.env` and fill in the required values.
    ```sh
    cp .env.sample .env
    ```

   Example `.env` file:
    ```dotenv
    AWS_ACCOUNT_ID=123456789012
    AWS_REGION=ap-northeast-1
    IMAGE_REPOSITORY=nginx:alpine
    ```

2. **Create ECR repository** (if it doesn't exist):
   Manually create the ECR repository if it doesn't exist:
    ```sh
    aws ecr create-repository --repository-name nginx --region ap-northeast-1
    ```

3. **Login to ECR and push images**:
    ```sh
    make push
    ```
