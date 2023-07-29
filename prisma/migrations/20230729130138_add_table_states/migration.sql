-- CreateEnum
CREATE TYPE "IndexedType" AS ENUM ('LOG', 'TOKEN');

-- AlterTable
ALTER TABLE "Logs" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());

-- AlterTable
ALTER TABLE "Tokens" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());

-- CreateTable
CREATE TABLE "States" (
    "id" INTEGER NOT NULL,
    "chain" "Chain" NOT NULL,
    "indexed_type" "IndexedType" NOT NULL,
    "indexed_block" BIGINT NOT NULL,

    CONSTRAINT "States_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "States_chain_key" ON "States"("chain");
