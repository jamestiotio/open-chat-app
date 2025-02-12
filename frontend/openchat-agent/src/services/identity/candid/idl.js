export const idlFactory = ({ IDL }) => {
  const CheckAuthPrincipalResponse = IDL.Variant({
    'NotFound' : IDL.Null,
    'Success' : IDL.Null,
    'Legacy' : IDL.Null,
  });
  const PublicKey = IDL.Vec(IDL.Nat8);
  const TimestampNanoseconds = IDL.Nat64;
  const GetDelegationArgs = IDL.Record({
    'session_key' : PublicKey,
    'expiration' : TimestampNanoseconds,
  });
  const SignedDelegation = IDL.Record({
    'signature' : IDL.Vec(IDL.Nat8),
    'delegation' : IDL.Record({
      'pubkey' : PublicKey,
      'expiration' : TimestampNanoseconds,
    }),
  });
  const GetDelegationResponse = IDL.Variant({
    'NotFound' : IDL.Null,
    'Success' : SignedDelegation,
  });
  const MigrateLegacyPrincipalResponse = IDL.Variant({
    'NotFound' : IDL.Null,
    'Success' : IDL.Record({ 'new_principal' : IDL.Principal }),
    'InternalError' : IDL.Text,
    'AlreadyMigrated' : IDL.Null,
  });
  const Nanoseconds = IDL.Nat64;
  const PrepareDelegationArgs = IDL.Record({
    'session_key' : PublicKey,
    'max_time_to_live' : IDL.Opt(Nanoseconds),
  });
  const PrepareDelegationResponse = IDL.Variant({
    'NotFound' : IDL.Null,
    'Success' : IDL.Record({
      'user_key' : PublicKey,
      'expiration' : TimestampNanoseconds,
    }),
  });
  return IDL.Service({
    'check_auth_principal' : IDL.Func(
        [IDL.Record({})],
        [CheckAuthPrincipalResponse],
        ['query'],
      ),
    'get_delegation' : IDL.Func(
        [GetDelegationArgs],
        [GetDelegationResponse],
        ['query'],
      ),
    'migrate_legacy_principal' : IDL.Func(
        [IDL.Record({})],
        [MigrateLegacyPrincipalResponse],
        [],
      ),
    'prepare_delegation' : IDL.Func(
        [PrepareDelegationArgs],
        [PrepareDelegationResponse],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
