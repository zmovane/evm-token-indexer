-- CreateEnum
CREATE TYPE "Chain" AS ENUM ('ETHEREUM_GOERLI', 'POLYGON_MUMBAI', 'OPBNB_TESTNET', 'BNBCHAIN_TESTNET', 'ZKSYNC_ERA_TESTNET');

-- CreateEnum
CREATE TYPE "Standard" AS ENUM ('ERC721', 'ERC1155');

-- CreateEnum
CREATE TYPE "IndexedType" AS ENUM ('LOG', 'TOKEN');

-- CreateTable
CREATE TABLE "Logs" (
    "id" VARCHAR(36) NOT NULL DEFAULT (gen_random_uuid()),
    "tx_hash" VARCHAR(66) NOT NULL,
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "topics" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "address" VARCHAR(66) NOT NULL,
    "data" BYTEA NOT NULL,

    CONSTRAINT "Logs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "States" (
    "chain" "Chain" NOT NULL,
    "indexed_type" "IndexedType" NOT NULL,
    "indexed_block" BIGINT NOT NULL
);

-- CreateTable
CREATE TABLE "Tokens" (
    "id" VARCHAR(36) NOT NULL DEFAULT (gen_random_uuid()),
    "chain" "Chain" NOT NULL,
    "tokenId" VARCHAR(66) NOT NULL,
    "contract" VARCHAR(66) NOT NULL,
    "owner" VARCHAR(66) NOT NULL,
    "uri" VARCHAR(200),
    "standard" "Standard" NOT NULL,
    "indexed_block" BIGINT NOT NULL,

    CONSTRAINT "Tokens_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "Logs_block_number_idx" ON "Logs"("block_number");

-- CreateIndex
CREATE INDEX "Logs_tx_hash_idx" ON "Logs"("tx_hash");

-- CreateIndex
CREATE UNIQUE INDEX "Logs_block_number_log_index_key" ON "Logs"("block_number", "log_index");

-- CreateIndex
CREATE UNIQUE INDEX "States_chain_indexed_type_key" ON "States"("chain", "indexed_type");

-- CreateIndex
CREATE INDEX "Tokens_standard_idx" ON "Tokens"("standard");

-- CreateIndex
CREATE INDEX "Tokens_chain_idx" ON "Tokens"("chain");

-- CreateIndex
CREATE INDEX "Tokens_owner_idx" ON "Tokens"("owner");

-- CreateIndex
CREATE INDEX "Tokens_contract_idx" ON "Tokens"("contract");

-- CreateIndex
CREATE UNIQUE INDEX "Tokens_chain_tokenId_contract_key" ON "Tokens"("chain", "tokenId", "contract");
