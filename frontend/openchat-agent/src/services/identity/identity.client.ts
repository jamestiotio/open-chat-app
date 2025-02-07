import type { Identity, SignIdentity } from "@dfinity/agent";
import { idlFactory, type IdentityService } from "./candid/idl";
import { CandidService } from "../candidService";
import type {
    CheckAuthPrincipalResponse,
    GetDelegationResponse,
    MigrateLegacyPrincipalResponse,
    PrepareDelegationResponse,
} from "openchat-shared";
import {
    checkAuthPrincipalResponse,
    getDelegationResponse,
    migrateLegacyPrincipalResponse,
    prepareDelegationResponse,
} from "./mappers";

export class IdentityClient extends CandidService {
    private service: IdentityService;

    private constructor(identity: Identity, identityCanister: string, icUrl: string) {
        super(identity);

        this.service = this.createServiceClient<IdentityService>(idlFactory, identityCanister, {
            icUrl,
        });
    }

    static create(identity: Identity, identityCanister: string, icUrl: string): IdentityClient {
        return new IdentityClient(identity, identityCanister, icUrl);
    }

    checkAuthPrincipal(): Promise<CheckAuthPrincipalResponse> {
        return this.handleQueryResponse(
            () => this.service.check_auth_principal({}),
            checkAuthPrincipalResponse,
            {},
        );
    }

    migrateLegacyPrincipal(): Promise<MigrateLegacyPrincipalResponse> {
        return this.handleResponse(
            this.service.migrate_legacy_principal({}),
            migrateLegacyPrincipalResponse,
            {},
        );
    }

    prepareDelegation(): Promise<PrepareDelegationResponse> {
        const args = {
            session_key: new Uint8Array((this.identity as SignIdentity).getPublicKey().toDer()),
            max_time_to_live: [] as [] | [bigint],
        };
        return this.handleResponse(
            this.service.prepare_delegation(args),
            prepareDelegationResponse,
            args,
        );
    }

    getDelegation(sessionKey: Uint8Array, expiration: bigint): Promise<GetDelegationResponse> {
        const args = {
            session_key: sessionKey,
            expiration,
        };
        return this.handleQueryResponse(
            () => this.service.get_delegation(args),
            getDelegationResponse,
            args,
        );
    }
}
