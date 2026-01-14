use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use log::{debug, info, error, warn};
use env_logger;

// 定义应用状态结构
#[derive(Clone)]
struct AppState {
    // 内存数据库 - 用户数据
    users: Arc<DashMap<String, User>>,
    // 内存数据库 - 传记项目数据
    biographies: Arc<DashMap<String, Biography>>,
    // 内存缓存 - 会话数据
    sessions: Arc<DashMap<String, Session>>,
}

// 用户结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    openid: String,      // 微信openid
    nickname: String,
    avatar: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// 传记项目结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Biography {
    id: String,
    user_id: String,
    title: String,
    description: Option<String>,
    content: String,
    status: BiographyStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// 传记状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
enum BiographyStatus {
    Draft,
    Published,
    Archived,
}

// 会话结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Session {
    session_id: String,
    user_id: String,
    expires_at: DateTime<Utc>,
}

// API响应结构体
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
    timestamp: DateTime<Utc>,
}

// 请求结构体
#[derive(Deserialize)]
struct CreateUserRequest {
    openid: String,
    nickname: String,
    avatar: Option<String>,
}

#[derive(Deserialize)]
struct CreateBiographyRequest {
    user_id: String,
    title: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct UpdateBiographyRequest {
    title: Option<String>,
    description: Option<String>,
    content: Option<String>,
}

// API响应辅助函数
fn ok_response<T>(data: T, message: String) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        message,
        timestamp: Utc::now(),
    })
}

fn error_response<T>(message: String) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: false,
        data: None,
        message,
        timestamp: Utc::now(),
    })
}

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();
    
    info!("启动传记写作助手后端服务...");
    
    // 初始化应用状态
    let state = AppState {
        users: Arc::new(DashMap::new()),
        biographies: Arc::new(DashMap::new()),
        sessions: Arc::new(DashMap::new()),
    };
    
    // 定义路由
    let app = Router::new()
        // 用户相关API
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user))
        
        // 传记项目相关API
        .route("/api/biographies", post(create_biography))
        .route("/api/biographies", get(list_biographies))
        .route("/api/biographies/:id", get(get_biography))
        .route("/api/biographies/:id", post(update_biography))
        
        // AI助手相关API
        .route("/api/ai/generate-outline", post(generate_outline))
        .route("/api/ai/generate-content", post(generate_content))
        .route("/api/ai/interview-questions", post(interview_questions))
        
        // 根路径
        .route("/", get(root_handler))
        
        // 注入应用状态
        .with_state(state);

    info!("服务器将在 http://0.0.0.0:3000 上运行");
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 根路径处理
async fn root_handler() -> &'static str {
    debug!("收到根路径请求");
    "传记写作助手后端服务 - API文档请参考相关接口"
}

// 创建用户
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    debug!("收到创建用户请求: {:?}", payload);
    
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let user = User {
        id: user_id.clone(),
        openid: payload.openid,
        nickname: payload.nickname,
        avatar: payload.avatar,
        created_at: now,
        updated_at: now,
    };
    
    state.users.insert(user_id.clone(), user.clone());
    
    info!("成功创建用户: {}", user_id);
    
    Ok(ok_response(user, "用户创建成功".to_string()))
}

// 获取用户信息
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    debug!("收到获取用户请求: {}", id);
    
    match state.users.get(&id) {
        Some(user_ref) => {
            info!("成功获取用户信息: {}", id);
            Ok(ok_response(user_ref.value().clone(), "获取用户信息成功".to_string()))
        }
        None => {
            warn!("未找到用户: {}", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

// 创建传记项目
async fn create_biography(
    State(state): State<AppState>,
    Json(payload): Json<CreateBiographyRequest>,
) -> Result<Json<ApiResponse<Biography>>, StatusCode> {
    debug!("收到创建传记项目请求: {:?}", payload);
    
    // 检查用户是否存在
    if !state.users.contains_key(&payload.user_id) {
        error!("创建传记项目失败：用户不存在 - {}", payload.user_id);
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let biography_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    let biography = Biography {
        id: biography_id.clone(),
        user_id: payload.user_id,
        title: payload.title,
        description: payload.description,
        content: "".to_string(),
        status: BiographyStatus::Draft,
        created_at: now,
        updated_at: now,
    };
    
    state.biographies.insert(biography_id.clone(), biography.clone());
    
    info!("成功创建传记项目: {}", biography_id);
    
    Ok(ok_response(biography, "传记项目创建成功".to_string()))
}

// 列出用户的传记项目
async fn list_biographies(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<Biography>>>, StatusCode> {
    debug!("收到列出传记项目请求: {:?}", params);
    
    let user_id = match params.get("user_id") {
        Some(id) => id,
        None => {
            error!("缺少必需参数: user_id");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    let mut user_biographies = Vec::new();
    for biography_ref in state.biographies.iter() {
        if biography_ref.value().user_id == *user_id {
            user_biographies.push(biography_ref.value().clone());
        }
    }
    
    info!("成功获取用户 {} 的传记项目列表，共 {} 个项目", user_id, user_biographies.len());
    
    Ok(ok_response(user_biographies, "获取传记项目列表成功".to_string()))
}

// 获取单个传记项目
async fn get_biography(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Biography>>, StatusCode> {
    debug!("收到获取传记项目请求: {}", id);
    
    match state.biographies.get(&id) {
        Some(biography_ref) => {
            info!("成功获取传记项目: {}", id);
            Ok(ok_response(biography_ref.value().clone(), "获取传记项目成功".to_string()))
        }
        None => {
            warn!("未找到传记项目: {}", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

// 更新传记项目
async fn update_biography(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBiographyRequest>,
) -> Result<Json<ApiResponse<Biography>>, StatusCode> {
    debug!("收到更新传记项目请求: {}, {:?}", id, payload);
    
    match state.biographies.get_mut(&id) {
        Some(mut biography_ref) => {
            if let Some(title) = payload.title {
                biography_ref.title = title;
            }
            if let Some(description) = payload.description {
                biography_ref.description = Some(description);
            }
            if let Some(content) = payload.content {
                biography_ref.content = content;
            }
            
            biography_ref.updated_at = Utc::now();
            
            let updated_biography = biography_ref.clone();
            info!("成功更新传记项目: {}", id);
            
            Ok(ok_response(updated_biography, "传记项目更新成功".to_string()))
        }
        None => {
            error!("更新传记项目失败：传记项目不存在 - {}", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

// AI生成大纲
async fn generate_outline(
    State(_state): State<AppState>,
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    debug!("收到AI生成大纲请求: {:?}", payload);
    
    // 这里应该是调用QWen API的逻辑
    // 模拟返回一个大纲
    let outline = r#"{
        "title": "传记大纲",
        "chapters": [
            {"chapter_number": 1, "title": "早年生活", "summary": "描述主人公的出生背景和童年经历"},
            {"chapter_number": 2, "title": "求学之路", "summary": "讲述教育经历和成长过程"},
            {"chapter_number": 3, "title": "职业生涯", "summary": "记录工作经历和职业发展"},
            {"chapter_number": 4, "title": "重要事件", "summary": "回顾人生中的重大转折点"},
            {"chapter_number": 5, "title": "成就与影响", "summary": "总结取得的成就和社会影响"},
            {"chapter_number": 6, "title": "晚年生活", "summary": "描述晚年时光和人生感悟"}
        ]
    }"#.to_string();
    
    info!("AI成功生成传记大纲");
    
    Ok(ok_response(outline, "大纲生成成功".to_string()))
}

// AI生成内容
async fn generate_content(
    State(_state): State<AppState>,
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    debug!("收到AI生成内容请求: {:?}", payload);
    
    // 这里应该是调用QWen API的逻辑
    // 模拟返回一些内容
    let content = "这是由AI生成的传记内容示例。在实际实现中，这里会调用QWen API来生成高质量的传记内容。".to_string();
    
    info!("AI成功生成传记内容");
    
    Ok(ok_response(content, "内容生成成功".to_string()))
}

// AI生成采访问题
async fn interview_questions(
    State(_state): State<AppState>,
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    debug!("收到AI生成采访问题请求: {:?}", payload);
    
    // 这里应该是调用QWen API的逻辑
    // 模拟返回一些采访问题
    let questions = r#"[
        "您能介绍一下自己的童年和家庭背景吗？",
        "在您的成长过程中，哪些人对您产生了深远的影响？",
        "您人生中最重要的转折点是什么时候？",
        "面对困难时，您是如何坚持下来的？",
        "您认为自己最大的成就是什么？",
        "对于年轻一代，您有什么建议或寄语？"
    ]"#.to_string();
    
    info!("AI成功生成采访问题");
    
    Ok(ok_response(questions, "采访问题生成成功".to_string()))
}