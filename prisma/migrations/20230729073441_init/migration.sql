-- CreateEnum
CREATE TYPE "Chain" AS ENUM ('ETHEREUM_GOERLI', 'POLYGON_MUMBAI', 'OPBNB_TESTNET', 'BNBCHAIN_TESTNET', 'ZKSYNC_ERA_TESTNET');

-- CreateEnum
CREATE TYPE "Standard" AS ENUM ('ERC721', 'ERC1155');

-- CreateTable
CREATE TABLE "Log" (
    "id" TEXT NOT NULL,
    "tx_hash" VARCHAR(66) NOT NULL,
    "block_number" BIGINT NOT NULL,
    "topics" VARCHAR(66)[],
    "data" BYTEA NOT NULL,

    CONSTRAINT "Log_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Token" (
    "id" TEXT NOT NULL,
    "chain" "Chain" NOT NULL,
    "tokenId" TEXT NOT NULL,
    "contract" VARCHAR(44) NOT NULL,
    "owner" VARCHAR(44) NOT NULL,
    "uri" VARCHAR(200) NOT NULL,
    "standard" "Standard" NOT NULL,

    CONSTRAINT "Token_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Token_chain_tokenId_contract_key" ON "Token"("chain", "tokenId", "contract");
