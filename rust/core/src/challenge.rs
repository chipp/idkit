//! `ChallengeKit` request and response types
//!
//! Mirrors the TypeScript types from `worldcoin/challengekit-js`:
//! - `packages/core/src/types/challenge.ts`
//! - `packages/core/src/types/response.ts`
//! - `packages/core/src/types/errors.ts`

use serde::{Deserialize, Serialize};

use crate::types::VerificationLevel;

// ─────────────────────────────────────────────────────────────────────────────
// Request types
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level request type for a challenge configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    Verify,
    DeepFace,
}

/// Encryption parameters for a face image challenge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct FaceImageEncryption {
    pub algorithm: String,
    pub key: String,
    pub iv: String,
}

/// Data payload for a face image challenge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct FaceImageChallengeData {
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<FaceImageEncryption>,
}

/// Username fields that can be requested in a World App username challenge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
#[serde(rename_all = "snake_case")]
pub enum UsernameField {
    Username,
}

/// Expected values for validating a World App username challenge response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ExpectedUsernameValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Data payload for a World App username challenge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct WorldAppUsernameChallengeData {
    pub requested_fields: Vec<UsernameField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_values: Option<ExpectedUsernameValues>,
}

/// A single challenge item — discriminated union matching the JS `ChallengeType`
///
/// Serializes as `{ "type": "<variant>", "data": { ... } }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChallengeType {
    FaceImage {
        data: FaceImageChallengeData,
    },
    WorldAppUsername {
        data: WorldAppUsernameChallengeData,
    },
}

/// World ID proof configuration embedded in a challenge config
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct WorldIDProof {
    pub app_id: String,
    pub action: String,
    pub signal: String,
    pub verification_level: VerificationLevel,
}

/// Full challenge configuration sent to World App
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ChallengeConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub request_type: RequestType,
    pub world_id_proof: WorldIDProof,
    pub challenges: Vec<ChallengeType>,
}

/// Lightweight payload used to group challenges by ID
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ChallengePayload {
    pub challenge_id: String,
    pub challenges: Vec<ChallengeType>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Response types
// ─────────────────────────────────────────────────────────────────────────────

/// Sign-In with Ethereum proof included in a username response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct SiweProof {
    /// Serialized EIP-4361 message (string form)
    pub message: String,
    pub signature: String,
}

/// Data payload for a face image challenge response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct DeepFaceResponseData {
    pub verified: bool,
}

/// Data payload for a World App username challenge response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct WorldAppUsernameResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siwe: Option<SiweProof>,
}

/// A single challenge response — discriminated union matching the JS `ChallengeResponseType`
///
/// Serializes as `{ "type": "<variant>", "data": { ... } }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChallengeResponseType {
    FaceImage {
        data: DeepFaceResponseData,
    },
    WorldAppUsername {
        data: WorldAppUsernameResponseData,
    },
}

/// World ID proof data included in a `ChallengeResponse`
///
/// Mirrors the JS `ISuccessResult` interface from `@worldcoin/idkit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct WorldIdSuccessResult {
    pub proof: String,
    pub nullifier_hash: String,
    pub merkle_root: String,
    pub verification_level: String,
}

/// Complete challenge response returned after verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ChallengeResponse {
    pub challenge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_id_proof: Option<WorldIdSuccessResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<ChallengeResponseType>>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Error types
// ─────────────────────────────────────────────────────────────────────────────

/// Known error codes for challenge operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::AsRefStr)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ChallengeErrorCode {
    Unknown,
    AlreadyInstalled,
    MissingAppId,
    InvalidResponse,
    VerificationFailed,
    ConnectionFailed,
    ValidationFailed,
}

/// Structured error returned from a challenge operation
///
/// The `code` field is a plain `String` to match the JS type `ChallengeErrorCode | string`,
/// which allows unknown/future error codes. Use [`ChallengeErrorCode`] constants for
/// well-known values.
///
/// `details` is an arbitrary JSON object; FFI bindings are not generated for this type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ChallengeError {
    /// Creates a new error from a known error code
    #[must_use]
    pub fn new(code: ChallengeErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a new error with extra detail context
    #[must_use]
    pub fn with_details(
        code: ChallengeErrorCode,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            details: Some(details),
        }
    }
}

/// Proof data included in a successful [`ChallengeVerificationResult`]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeVerificationProofData {
    pub proof: String,
    pub nullifier_hash: String,
    pub merkle_root: String,
    pub verification_level: String,
}

/// Result of verifying a challenge response
///
/// Mirrors `ChallengeVerificationErrorResult` from the JS SDK.
/// FFI derives are omitted because the nested [`ChallengeError`] contains
/// a [`serde_json::Value`] field which is not supported by `UniFFI`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeVerificationResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ChallengeError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ChallengeVerificationProofData>,
}

impl ChallengeVerificationResult {
    /// Builds a successful result with proof data
    #[must_use]
    pub fn success(data: ChallengeVerificationProofData) -> Self {
        Self { success: true, error: None, data: Some(data) }
    }

    /// Builds a failure result with an error
    #[must_use]
    pub fn failure(error: ChallengeError) -> Self {
        Self { success: false, error: Some(error), data: None }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_type_face_image_round_trip() {
        let challenge = ChallengeType::FaceImage {
            data: FaceImageChallengeData {
                image_url: "https://example.com/img.bin".to_string(),
                encryption: Some(FaceImageEncryption {
                    algorithm: "aes-256-gcm".to_string(),
                    key: "base64key==".to_string(),
                    iv: "base64iv==".to_string(),
                }),
            },
        };

        let json = serde_json::to_string(&challenge).unwrap();
        assert!(json.contains(r#""type":"face_image""#));
        assert!(json.contains(r#""image_url""#));

        let roundtrip: ChallengeType = serde_json::from_str(&json).unwrap();
        assert_eq!(challenge, roundtrip);
    }

    #[test]
    fn test_challenge_type_world_app_username_round_trip() {
        let challenge = ChallengeType::WorldAppUsername {
            data: WorldAppUsernameChallengeData {
                requested_fields: vec![UsernameField::Username],
                expected_values: Some(ExpectedUsernameValues {
                    username: Some("alice".to_string()),
                }),
            },
        };

        let json = serde_json::to_string(&challenge).unwrap();
        assert!(json.contains(r#""type":"world_app_username""#));

        let roundtrip: ChallengeType = serde_json::from_str(&json).unwrap();
        assert_eq!(challenge, roundtrip);
    }

    #[test]
    fn test_challenge_config_round_trip() {
        let config = ChallengeConfig {
            id: "challenge-123".to_string(),
            request_type: RequestType::DeepFace,
            world_id_proof: WorldIDProof {
                app_id: "app_staging_abc".to_string(),
                action: "deepface".to_string(),
                signal: "user-signal".to_string(),
                verification_level: VerificationLevel::Orb,
            },
            challenges: vec![ChallengeType::FaceImage {
                data: FaceImageChallengeData {
                    image_url: "https://example.com/img.bin".to_string(),
                    encryption: None,
                },
            }],
        };

        let json = serde_json::to_string(&config).unwrap();
        // `type` field (not `request_type`) in the JSON output
        assert!(json.contains(r#""type":"deep_face""#));

        let roundtrip: ChallengeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, roundtrip);
    }

    #[test]
    fn test_challenge_response_type_round_trip() {
        let response = ChallengeResponseType::FaceImage {
            data: DeepFaceResponseData { verified: true },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains(r#""type":"face_image""#));
        assert!(json.contains(r#""verified":true"#));

        let roundtrip: ChallengeResponseType = serde_json::from_str(&json).unwrap();
        assert_eq!(response, roundtrip);
    }

    #[test]
    fn test_username_response_with_siwe_round_trip() {
        let response = ChallengeResponseType::WorldAppUsername {
            data: WorldAppUsernameResponseData {
                username: Some("alice".to_string()),
                siwe: Some(SiweProof {
                    message: "Sign this message".to_string(),
                    signature: "0xdeadbeef".to_string(),
                }),
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains(r#""type":"world_app_username""#));

        let roundtrip: ChallengeResponseType = serde_json::from_str(&json).unwrap();
        assert_eq!(response, roundtrip);
    }

    #[test]
    fn test_challenge_verification_result_success() {
        let result = ChallengeVerificationResult::success(ChallengeVerificationProofData {
            proof: "0xproof".to_string(),
            nullifier_hash: "0xnull".to_string(),
            merkle_root: "0xroot".to_string(),
            verification_level: "orb".to_string(),
        });

        assert!(result.success);
        assert!(result.error.is_none());
        assert!(result.data.is_some());

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""success":true"#));
    }

    #[test]
    fn test_challenge_verification_result_failure() {
        let result = ChallengeVerificationResult::failure(ChallengeError::new(
            ChallengeErrorCode::ValidationFailed,
            "username mismatch",
        ));

        assert!(!result.success);
        assert!(result.error.is_some());
        assert_eq!(result.error.as_ref().unwrap().code, "validation_failed");

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""success":false"#));
        assert!(json.contains("validation_failed"));
    }

    #[test]
    fn test_challenge_error_code_display() {
        assert_eq!(ChallengeErrorCode::Unknown.to_string(), "unknown");
        assert_eq!(ChallengeErrorCode::ValidationFailed.to_string(), "validation_failed");
        assert_eq!(ChallengeErrorCode::ConnectionFailed.to_string(), "connection_failed");
    }

    #[test]
    fn test_face_image_no_encryption_omitted() {
        let challenge = ChallengeType::FaceImage {
            data: FaceImageChallengeData {
                image_url: "https://example.com/img.bin".to_string(),
                encryption: None,
            },
        };
        let json = serde_json::to_string(&challenge).unwrap();
        assert!(!json.contains("encryption"));
    }
}
