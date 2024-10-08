use crate::ResultCode;

pub fn integer_to_error_code(integer: i32) -> Result<ResultCode, ()>{
    match integer {
        x if x == ResultCode::CEF_RESULT_CODE_NORMAL_EXIT as i32 => Ok(ResultCode::CEF_RESULT_CODE_NORMAL_EXIT),
        x if x == ResultCode::CEF_RESULT_CODE_KILLED as i32 => Ok(ResultCode::CEF_RESULT_CODE_KILLED),
        x if x == ResultCode::CEF_RESULT_CODE_HUNG as i32 => Ok(ResultCode::CEF_RESULT_CODE_HUNG),
        x if x == ResultCode::CEF_RESULT_CODE_KILLED_BAD_MESSAGE as i32 => Ok(ResultCode::CEF_RESULT_CODE_KILLED_BAD_MESSAGE),
        x if x == ResultCode::CEF_RESULT_CODE_GPU_DEAD_ON_ARRIVAL as i32 => Ok(ResultCode::CEF_RESULT_CODE_GPU_DEAD_ON_ARRIVAL),
        x if x == ResultCode::CEF_RESULT_CODE_CHROME_FIRST as i32 => Ok(ResultCode::CEF_RESULT_CODE_CHROME_FIRST),
        x if x == ResultCode::CEF_RESULT_CODE_MISSING_DATA as i32 => Ok(ResultCode::CEF_RESULT_CODE_MISSING_DATA),
        x if x == ResultCode::CEF_RESULT_CODE_UNSUPPORTED_PARAM as i32 => Ok(ResultCode::CEF_RESULT_CODE_UNSUPPORTED_PARAM),
        x if x == ResultCode::CEF_RESULT_CODE_PROFILE_IN_USE as i32 => Ok(ResultCode::CEF_RESULT_CODE_PROFILE_IN_USE),
        x if x == ResultCode::CEF_RESULT_CODE_PACK_EXTENSION_ERROR as i32 => Ok(ResultCode::CEF_RESULT_CODE_PACK_EXTENSION_ERROR),
        x if x == ResultCode::CEF_RESULT_CODE_NORMAL_EXIT_PROCESS_NOTIFIED as i32 => Ok(ResultCode::CEF_RESULT_CODE_NORMAL_EXIT_PROCESS_NOTIFIED),
        x if x == ResultCode::CEF_RESULT_CODE_INVALID_SANDBOX_STATE as i32 => Ok(ResultCode::CEF_RESULT_CODE_INVALID_SANDBOX_STATE),
        x if x == ResultCode::CEF_RESULT_CODE_CLOUD_POLICY_ENROLLMENT_FAILED as i32 => Ok(ResultCode::CEF_RESULT_CODE_CLOUD_POLICY_ENROLLMENT_FAILED),
        x if x == ResultCode::CEF_RESULT_CODE_GPU_EXIT_ON_CONTEXT_LOST as i32 => Ok(ResultCode::CEF_RESULT_CODE_GPU_EXIT_ON_CONTEXT_LOST),
        x if x == ResultCode::CEF_RESULT_CODE_NORMAL_EXIT_PACK_EXTENSION_SUCCESS as i32 => Ok(ResultCode::CEF_RESULT_CODE_NORMAL_EXIT_PACK_EXTENSION_SUCCESS),
        x if x == ResultCode::CEF_RESULT_CODE_SYSTEM_RESOURCE_EXHAUSTED as i32 => Ok(ResultCode::CEF_RESULT_CODE_SYSTEM_RESOURCE_EXHAUSTED),
        x if x == ResultCode::CEF_RESULT_CODE_CHROME_LAST as i32 => Ok(ResultCode::CEF_RESULT_CODE_CHROME_LAST),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_FIRST as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_FIRST),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_DROPTOKEN as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_DROPTOKEN),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_FLUSHANDLES as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_FLUSHANDLES),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_CACHEDISABLE as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_CACHEDISABLE),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_CLOSEHANDLES as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_CLOSEHANDLES),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_MITIGATION as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_MITIGATION),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_MEMORY_EXCEEDED as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_MEMORY_EXCEEDED),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_WARMUP as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_WARMUP),
        x if x == ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_LAST as i32 => Ok(ResultCode::CEF_RESULT_CODE_SANDBOX_FATAL_LAST),
        _ => Err(()),
    }
}