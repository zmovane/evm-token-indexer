/*
  Warnings:

  - The primary key for the `Logs` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to alter the column `id` on the `Logs` table. The data in that column could be lost. The data in that column will be cast from `VarChar(64)` to `VarChar(36)`.
  - The primary key for the `Tokens` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to alter the column `id` on the `Tokens` table. The data in that column could be lost. The data in that column will be cast from `VarChar(64)` to `VarChar(36)`.
  - You are about to alter the column `tokenId` on the `Tokens` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(66)`.

*/
-- AlterTable
ALTER TABLE "Logs" DROP CONSTRAINT "Logs_pkey",
ALTER COLUMN "id" SET DEFAULT (gen_random_uuid()),
ALTER COLUMN "id" SET DATA TYPE VARCHAR(36),
ALTER COLUMN "address" SET DATA TYPE VARCHAR(66),
ADD CONSTRAINT "Logs_pkey" PRIMARY KEY ("id");

-- AlterTable
ALTER TABLE "Tokens" DROP CONSTRAINT "Tokens_pkey",
ALTER COLUMN "id" SET DEFAULT (gen_random_uuid()),
ALTER COLUMN "id" SET DATA TYPE VARCHAR(36),
ALTER COLUMN "tokenId" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "contract" SET DATA TYPE VARCHAR(66),
ALTER COLUMN "owner" SET DATA TYPE VARCHAR(66),
ADD CONSTRAINT "Tokens_pkey" PRIMARY KEY ("id");
