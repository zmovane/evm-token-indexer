/*
  Warnings:

  - The primary key for the `States` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `id` on the `States` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Logs" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());

-- AlterTable
ALTER TABLE "States" DROP CONSTRAINT "States_pkey",
DROP COLUMN "id";

-- AlterTable
ALTER TABLE "Tokens" ALTER COLUMN "id" SET DEFAULT (gen_random_uuid());
