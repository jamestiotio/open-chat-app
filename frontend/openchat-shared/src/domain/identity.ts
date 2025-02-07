import { Delegation } from "@dfinity/identity";
import type { Signature } from "@dfinity/agent";

export type HasIdentity = {
    id: string;
};

export type CheckAuthPrincipalResponse =
    | { kind: "success" }
    | { kind: "legacy" }
    | { kind: "not_found" };

export type MigrateLegacyPrincipalResponse =
    | { kind: "success"; newPrincipal: string }
    | { kind: "already_migrated" }
    | { kind: "not_found" }
    | { kind: "internal_error"; error: string };

export type PrepareDelegationResponse =
    | {
          kind: "success";
          userKey: Uint8Array;
          expiration: bigint;
      }
    | { kind: "not_found" };

export type GetDelegationResponse =
    | {
          kind: "success";
          delegation: Delegation;
          signature: Signature;
      }
    | { kind: "not_found" };
