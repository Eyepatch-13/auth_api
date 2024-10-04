use core::str;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

use crate::models::{UserRole, User};

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min=1, message="Name is Required"))]
    pub name: String,

    #[validate(
        length(min=1, message="Email is required"),
        email(message="Email is invalid")
    )]
    pub email: String,

    #[validate(length(min=8, message="Password should be at least 8 characters"))]
    pub password: String,

    #[validate(
        length(min=1, message="Confirm password is required"),
        must_match(other="password", message="Passwords do not match")
    )]
    #[serde(rename="passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Debug, Default, Validate, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(
        length(min=1, message="Email is required"),
        email(message="Email is invalid")
    )]
    pub email: String,

    #[validate(length(min=8, message="Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min=1))]
    pub page: Option<usize>,

    #[validate(range(min=1, max=50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename="createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename="updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String, 
    pub token: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default, Clone)]
pub struct NameUpdateDto {
    #[validate(length(min=1, message="Name is required"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct RoleUpdateDto {
    #[validate(custom(function = "validate_user_role"))]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_role")),
    }
}

#[derive(Debug, Default, Clone, Validate, Deserialize, Serialize)]
pub struct UserPasswordUpdateDto {
    #[validate(length(min=8, message="Password must be at least 8 characters"))]
    pub new_password: String,

    #[validate(
        length(min=8, message= "Confirm password must be at least 8 characters"),
        must_match(other="new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(length(min=1, message="Old password is required"))]
    pub old_password: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min=1, message="Email is required"))]
    pub token: String,
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct ForgotPasswordRequestDto {
    #[validate(length(min=1, message= "Email is required"))]
    pub email: String,
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min=1, message="Token is Required"))]
    pub token: String,

    #[validate(length(min=8, message="New password must be at least 8 characters"))]
    pub new_password: String,

    #[validate(
        length(min=8, message="New password confirm must be at least 8 characters"),
        must_match(other="new_password", message="New passwords do not match")
    )]
        pub new_password_confirm: String,
}
