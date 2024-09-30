import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StudentInformationSystem } from "../target/types/student_information_system";
import { expect } from "chai";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("student-information-system", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.StudentInformationSystem as Program<StudentInformationSystem>;

  const student = {
    name: "merve keser",
    biography: 'I\'m Web3 developer'
  }

  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  )

  it("Initializes the token!", async () => {
    const tx = await program.methods
      .initializeTokenMint()
      .rpc()
  });

  it("Add Student!", async () => {
    const tokenAccount = await getAssociatedTokenAddress(
      mint,
      provider.wallet.publicKey
    );

    const tx = await program.methods
      .addStudent(student.name, student.biography)
      .rpc()

    const account = await program.account.studentAccount.fetch(studentPda);
    expect(account.name).to.equal(student.name);
    expect(account.biography).to.equal(student.biography);
    expect(account.owner.toString()).to.equal(provider.wallet.publicKey.toString());

    const userAta = await getAccount(provider.connection, tokenAccount);
    expect(Number(userAta.amount)).to.equal((10*10)^6);

  });

  it("Update Student!", async () => {
    const newBiography = 'I\'m interested in Solana and Rust';

    const tx = await program.methods
      .updateStudent(student.name, newBiography)
      .rpc()

    const account = await program.account.studentAccount.fetch(studentPda);
    expect(account.name).to.equal(student.name);
    expect(account.biography).to.equal(newBiography);
    expect(account.owner.toString()).to.equal(provider.wallet.publicKey.toString());

  });

  it("Delete Student!", async () => {
    const tx = await program.methods
      .deleteStudent(student.name)
      .rpc()
  });
});
