From Coq import List.

(** Multi line comments 
    (* comments inside *)
    Corinductive Ltac
    Module Section Qed.
*)

Definition test := 1.

Record test := create {
    test: test
}.

Lemma test:
    forall i, i < 100.
Proof.
    intros. (* Test*)
Qed.