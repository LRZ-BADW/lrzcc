use crate::error::{AuthOnlyError, NotFoundOnlyError};
use lrzcc_wire::user::User;

pub fn require_admin_user(user: &User) -> Result<(), AuthOnlyError> {
    if !user.is_staff {
        return Err(AuthOnlyError::AuthorizationError(
            "Admin privileges required".to_string(),
        ));
    }
    Ok(())
}

pub fn require_admin_user_or_return_not_found(
    user: &User,
) -> Result<(), NotFoundOnlyError> {
    if !user.is_staff {
        return Err(NotFoundOnlyError::NotFoundError);
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

pub fn require_master_user_or_return_not_found(
    user: &User,
    project_id: u32,
) -> Result<(), NotFoundOnlyError> {
    if !user.is_staff && (user.role != 2 || user.project != project_id) {
        return Err(NotFoundOnlyError::NotFoundError);
    }
    Ok(())
}

pub fn require_project_user(
    user: &User,
    project_id: u32,
) -> Result<(), AuthOnlyError> {
    if !user.is_staff && user.project != project_id {
        return Err(AuthOnlyError::AuthorizationError(
            "Must be admin or user of respective project".to_string(),
        ));
    }
    Ok(())
}

pub fn require_project_user_or_return_not_found(
    user: &User,
    project_id: u32,
) -> Result<(), NotFoundOnlyError> {
    if !user.is_staff && user.project != project_id {
        return Err(NotFoundOnlyError::NotFoundError);
    }
    Ok(())
}
