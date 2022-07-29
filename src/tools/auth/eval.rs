// TODO: auth eval system
// - GET https://jwxt.nwpu.edu.cn/student/for-std/evaluation/summative with SESSION & __pstsid__
// - Regex https://jwxt.nwpu.edu.cn/evaluation-student-frontend/#/sso-login?token=${JWT}
// - POST https://jwxt.nwpu.edu.cn/evaluation-student-backend/api/v1/evaluation/token-check with {"token":"${JWT}"}
// - Parse res["data"]["token"] as ${TOKEN}
// Visit with
// - Authorization: ${TOKEN}
// - Cookie: student_evaluation_token=${TOKEN}
