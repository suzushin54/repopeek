# .envファイルを読み込む
include .env
export $(shell sed 's/=.*//' .env)

# Open documentation with rustdoc
.PHONY: doc
doc:
	cargo doc --no-deps --open

# AWS ECRリポジトリURIを指定
ECR_URI := $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

# 環境変数からリポジトリ名とタグを取得
REPOSITORY := $(shell echo $(IMAGE_REPOSITORY) | cut -d ':' -f 1)
TAG := $(shell echo $(IMAGE_REPOSITORY) | cut -d ':' -f 2)

# ECRリポジトリにログイン
.PHONY: login
login:
	@aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(ECR_URI)

# イメージをECRにプッシュ
.PHONY: push
push: login
	@docker tag $(REPOSITORY):$(TAG) $(ECR_URI)/$(REPOSITORY):$(TAG)
	@docker push $(ECR_URI)/$(REPOSITORY):$(TAG)
