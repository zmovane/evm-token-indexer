-- CreateEnum
CREATE TYPE "Chain" AS ENUM ('ETHEREUM_GOERLI', 'POLYGON_MUMBAI', 'OPBNB_TESTNET', 'BNBCHAIN_TESTNET', 'ZKSYNC_ERA_TESTNET');

-- CreateEnum
CREATE TYPE "Standard" AS ENUM ('ERC721', 'ERC1155');

-- CreateTable
CREATE TABLE "Logs" (
    "id" VARCHAR(64) NOT NULL DEFAULT (gen_random_uuid()),
    "tx_hash" VARCHAR(66) NOT NULL,
    "block_number" BIGINT NOT NULL,
    "log_index" VARCHAR(66) NOT NULL,
    "topics" VARCHAR(66)[],
    "data" BYTEA NOT NULL,

    CONSTRAINT "Logs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Tokens" (
    "id" VARCHAR(64) NOT NULL DEFAULT (gen_random_uuid()),
    "chain" "Chain" NOT NULL,
    "tokenId" TEXT NOT NULL,
    "contract" VARCHAR(44) NOT NULL,
    "owner" VARCHAR(44) NOT NULL,
    "uri" VARCHAR(200) NOT NULL,
    "standard" "Standard" NOT NULL,

    CONSTRAINT "Tokens_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "Logs_block_number_idx" ON "Logs"("block_number");

-- CreateIndex
CREATE INDEX "Logs_tx_hash_idx" ON "Logs"("tx_hash");

-- CreateIndex
CREATE UNIQUE INDEX "Logs_block_number_log_index_key" ON "Logs"("block_number", "log_index");

-- CreateIndex
CREATE INDEX "Tokens_standard_idx" ON "Tokens"("standard");

-- CreateIndex
CREATE INDEX "Tokens_chain_idx" ON "Tokens"("chain");

-- CreateIndex
CREATE INDEX "Tokens_contract_idx" ON "Tokens"("contract");

-- CreateIndex
CREATE UNIQUE INDEX "Tokens_chain_tokenId_contract_key" ON "Tokens"("chain", "tokenId", "contract");
