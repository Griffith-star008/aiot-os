# Versioning & API Policy

The AIoT Framework adheres strictly to [Semantic Versioning 2.0.0](https://semver.org/).

## 1. SemVer Rules
Given a version number `MAJOR.MINOR.PATCH`:
- **MAJOR**: Incompatible API changes.
- **MINOR**: Add functionality in a backwards compatible manner.
- **PATCH**: Backwards compatible bug fixes.

## 2. API Stability Guarantee
- Code located in `api/` directories or exported publicly is considered STABLE in `v1.x`.
- Code flagged as `#[cfg(feature = "experimental")]` is UNSTABLE and may change in PATCH releases.

## 3. Deprecation Policy
Before a feature is removed in a `MAJOR` release:
- It must be marked with `#[deprecated(since = "X.Y.Z", note = "Use Foo instead")]`.
- It must remain in the codebase for at least one full `MINOR` release cycle before removal.

## 4. Backward Compatibility
- Database schemas (Storage Layer) migrations must always provide an upgrade path.
- Network protocols (Transport Layer) must negotiate versions to support older clients.
