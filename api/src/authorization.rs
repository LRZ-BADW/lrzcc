use crate::error::AuthOnlyError;
use lrzcc_wire::user::User;

// TODO: maybe we need alternative calls that return a not found error to not leak information

pub fn require_admin_user(user: &User) -> Result<(), AuthOnlyError> {
    if !user.is_staff {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin privileges required".to_string(),
        ));
    }
    Ok(())
}

pub fn require_master_user(
    user: &User,
    project_id: u32,
) -> Result<(), AuthOnlyError> {
    if !user.is_staff && (user.role != 2 || user.project != project_id) {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin or master user privileges for respective project required"
                .to_string(),
        ));
    }
    Ok(())
}

pub fn require_project_user(
    user: &User,
    project_id: u32,
) -> Result<(), AuthOnlyError> {
    if !user.is_staff && user.project != project_id {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin or master user privileges for respective project required"
                .to_string(),
        ));
    }
    Ok(())
}
