-- CreateIndex
CREATE INDEX "Log_block_number_idx" ON "Log"("block_number");

-- CreateIndex
CREATE INDEX "Log_tx_hash_idx" ON "Log"("tx_hash");

-- CreateIndex
CREATE INDEX "Token_standard_idx" ON "Token"("standard");

-- CreateIndex
CREATE INDEX "Token_chain_idx" ON "Token"("chain");

-- CreateIndex
CREATE INDEX "Token_contract_idx" ON "Token"("contract");
