/*
  Warnings:

  - Changed the type of `log_index` on the `Logs` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- AlterTable
ALTER TABLE "Logs" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid()),
DROP COLUMN "log_index",
ADD COLUMN     "log_index" BIGINT NOT NULL;

-- AlterTable
ALTER TABLE "Tokens" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());

-- CreateIndex
CREATE UNIQUE INDEX "Logs_block_number_log_index_key" ON "Logs"("block_number", "log_index");
