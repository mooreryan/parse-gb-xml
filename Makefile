TEST_DIR = "test_files"
TEST_OUTDIR = "test_files/output"
TEST_EXPECTED = "test_files/expected"

.PHONY: test

test:
	rm -r $(TEST_OUTDIR)/*; cargo run -- $(TEST_DIR)/sequences_3.gbx.xml $(TEST_OUTDIR)/genomes_OUTPUT.fa $(TEST_OUTDIR)/peptides_OUTPUT.faa && diff $(TEST_OUTDIR)/genomes_OUTPUT.fa $(TEST_EXPECTED)/genomes_OUTPUT.fa && diff $(TEST_OUTDIR)/peptides_OUTPUT.faa $(TEST_EXPECTED)/peptides_OUTPUT.faa