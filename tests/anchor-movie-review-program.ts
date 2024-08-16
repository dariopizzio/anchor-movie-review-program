import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token"
import { AnchorMovieReviewProgram } from "../target/types/anchor_movie_review_program";

describe("anchor-movie-review-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .AnchorMovieReviewProgram as Program<AnchorMovieReviewProgram>;

  const movie = {
    title: "Just a test movie",
    description: "Wow what a good movie it was real great",
    rating: 5,
  };

  const [moviePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(movie.title), provider.wallet.publicKey.toBuffer()],
    program.programId,
  );

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")]
    , program.programId
  );

  it("Movie review is added`", async () => {
    const pdaTokenAccount = await getAssociatedTokenAddress(mint, provider.wallet.publicKey);

    // We need to pass the associated token address (ata) as it cannot be inferred
    const tx = await program.methods.addMovieReview(movie.title, movie.description, movie.rating).accounts({ pdaTokenAccount: pdaTokenAccount }).rpc();

    const account = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title == account.title);
    expect(movie.description == account.description);
    expect(movie.rating == account.rating);
    expect(provider.wallet.publicKey == account.reviewer);

    const userAta = await getAccount(provider.connection, tokenAccount);
    expect(Number(userAta.amount)).to.equal((10 * 10) ^ 6);
  });

  it("Movie review is updated`", async () => {
    const newDescription = "new movie description";
    const newRating = 2;
    const tx = await program.methods.updateMovieReview(movie.title, newDescription, newRating).rpc();

    const account = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title == account.title);
    expect(newDescription == account.description);
    expect(newRating == account.rating);
    expect(provider.wallet.publicKey == account.reviewer);

  });

  it("Deletes a movie review", async () => {
    const tx = await program.methods.deleteMovieReview(movie.title).rpc();
  });

  it("Initializes the reward token", async () => {
    const tx = await program.methods.initializeTokenMint().rpc();
  });
});