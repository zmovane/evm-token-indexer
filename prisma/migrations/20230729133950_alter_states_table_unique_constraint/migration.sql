/*
  Warnings:

  - A unique constraint covering the columns `[chain,indexed_type]` on the table `States` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "States_chain_key";

-- AlterTable
ALTER TABLE "Logs" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());

-- AlterTable
ALTER TABLE "Tokens" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());

-- CreateIndex
CREATE UNIQUE INDEX "States_chain_indexed_type_key" ON "States"("chain", "indexed_type");
