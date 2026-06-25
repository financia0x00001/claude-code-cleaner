#![allow(dead_code)]

/// Chinese translations for UI display
#[allow(dead_code)]
pub fn translate_category_static(cat: &str) -> &'static str {
    match cat {
        "Projects" => "项目",
        "Debug Logs" => "调试日志",
        "File History" => "文件历史",
        "Telemetry" => "遥测数据",
        "Shell Snapshots" => "Shell 快照",
        "Plugins" => "插件",
        "Transcripts" => "会话记录",
        "Todos" => "待办事项",
        "Plans" => "计划",
        "Usage Data" => "使用数据",
        "Tasks" => "任务",
        "Paste Cache" => "剪贴板缓存",
        "Config Backups" => "配置备份",
        "History" => "历史记录",
        _ => "未知",
    }
}

pub fn translate_step_label(screen: &str) -> &'static str {
    match screen {
        "Scan" => "扫描",
        "Select" => "选择",
        "Projects" => "项目",
        "Preview" => "预览",
        "Clean" => "清理",
        _ => "未知",
    }
}

pub fn translate_dashboard_title() -> &'static str {
    "第一步：扫描"
}

pub fn translate_dashboard_header() -> &'static str {
    "~/.claude/"
}

pub fn translate_dashboard_total() -> &'static str {
    "总计"
}

pub fn translate_dashboard_files_label() -> &'static str {
    "个文件"
}

pub fn translate_dashboard_reclaimable() -> &'static str {
    "可回收（当前设置）"
}

pub fn translate_dashboard_btn_scan() -> &'static str {
    "扫描"
}

pub fn translate_dashboard_btn_browse() -> &'static str {
    "浏览"
}

pub fn translate_dashboard_btn_help() -> &'static str {
    "帮助"
}

pub fn translate_dashboard_no_data() -> &'static str {
    "按 [S] 扫描 ~/.claude/ 目录"
}

pub fn translate_dashboard_scanning() -> &'static str {
    "正在扫描..."
}

pub fn translate_select_title() -> &'static str {
    "选择与配置"
}

pub fn translate_select_categories_header() -> &'static str {
    "分类"
}

pub fn translate_select_config_header() -> &'static str {
    "  ── ~/.claude.json 清理 ────────────────────────────────"
}

pub fn translate_select_settings_header() -> &'static str {
    "  ── 设置 ──────────────────────────────────────────────"
}

pub fn translate_select_orphan_label() -> &'static str {
    "孤立项目"
}

pub fn translate_select_orphan_desc() -> &'static str {
    "移除已删除项目的条目"
}

pub fn translate_select_metrics_label() -> &'static str {
    "会话指标"
}

pub fn translate_select_metrics_desc() -> &'static str {
    "清除 lastModel/Cost/Duration/FPS/Lines 数据"
}

pub fn translate_select_cache_label() -> &'static str {
    "缓存标志与数据"
}

pub fn translate_select_cache_desc() -> &'static str {
    "移除 statsig/growth/遥测缓存"
}

pub fn translate_select_expiry_label() -> &'static str {
    "过期阈值（天）"
}

pub fn translate_select_dry_run_label() -> &'static str {
    "干跑模式（仅模拟）"
}

pub fn translate_select_selected() -> &'static str {
    "已选中："
}

pub fn translate_select_categories_label() -> &'static str {
    "个分类"
}

pub fn translate_select_all() -> &'static str {
    "全选"
}

pub fn translate_select_none() -> &'static str {
    "全不选"
}

pub fn translate_select_default() -> &'static str {
    "默认"
}

pub fn translate_projects_title() -> &'static str {
    "项目"
}

pub fn translate_projects_toggle() -> &'static str {
    " [空格] 切换  [a] 全选  [o] 仅孤立  [n] 取消  [/] 搜索"
}

pub fn translate_projects_search_editing() -> &'static str {
    "（输入中）"
}

pub fn translate_projects_search_active() -> &'static str {
    "搜索"
}

pub fn translate_projects_clear_hint() -> &'static str {
    "（按 Esc 清除）"
}

pub fn translate_projects_no_data() -> &'static str {
    "无扫描数据"
}

pub fn translate_projects_summary_placeholder() -> &'static str {
    ""
}

pub fn translate_preview_title() -> &'static str {
    "预览清理计划"
}

pub fn translate_preview_no_data() -> &'static str {
    "暂无扫描数据。请先运行扫描。"
}

pub fn translate_preview_clean_label() -> &'static str {
    "将清理"
}

pub fn translate_preview_skipped_label() -> &'static str {
    "跳过"
}

pub fn translate_preview_total_label() -> &'static str {
    "总计"
}

pub fn translate_preview_legend_clean() -> &'static str {
    " 将清理"
}

pub fn translate_preview_legend_unselected() -> &'static str {
    " 未选中（可匹配）"
}

pub fn translate_preview_legend_kept() -> &'static str {
    " 不受影响"
}

pub fn translate_preview_table_category() -> &'static str {
    "分类"
}

pub fn translate_preview_table_files() -> &'static str {
    "文件数"
}

pub fn translate_preview_table_size() -> &'static str {
    "大小"
}

pub fn translate_preview_table_action() -> &'static str {
    "操作"
}

pub fn translate_preview_action_trim() -> &'static str {
    "截断至500行"
}

pub fn translate_preview_action_delete() -> &'static str {
    "删除（大于{}天）"
}

pub fn translate_preview_action_delete_orphan() -> &'static str {
    "删除（大于{}天/孤立）"
}

pub fn translate_preview_config_json_label() -> &'static str {
    "配置 JSON"
}

pub fn translate_preview_config_clean_label() -> &'static str {
    "清理（{}）"
}

pub fn translate_preview_start_clean() -> &'static str {
    "开始清理"
}

pub fn translate_preview_start_dry_run() -> &'static str {
    "开始干跑"
}

pub fn translate_preview_go_back() -> &'static str {
    "返回"
}

pub fn translate_preview_dry_run_tag() -> &'static str {
    "【干跑模式】"
}

pub fn translate_cleaning_title() -> &'static str {
    "清理"
}

pub fn translate_cleaning_dry_run_title() -> &'static str {
    "清理【干跑】"
}

pub fn translate_cleaning_ready() -> &'static str {
    "准备就绪。前往预览并按回车开始。"
}

pub fn translate_cleaning_dry_run_progress() -> &'static str {
    "干跑中..."
}

pub fn translate_cleaning_cleaning_progress() -> &'static str {
    "清理中..."
}

pub fn translate_cleaning_dry_run_complete() -> &'static str {
    "干跑完成"
}

pub fn translate_cleaning_complete() -> &'static str {
    "完成"
}

pub fn translate_cleaning_would_free() -> &'static str {
    "将释放"
}

pub fn translate_cleaning_no_files_deleted() -> &'static str {
    "（未删除任何文件）"
}

pub fn translate_cleaning_total_freed() -> &'static str {
    "共释放"
}

pub fn translate_cleaning_quite() -> &'static str {
    "退出"
}

pub fn translate_cleaning_rescan() -> &'static str {
    "重新扫描"
}

pub fn translate_help_title() -> &'static str {
    "帮助"
}

pub fn translate_help_shortcuts_title() -> &'static str {
    "键盘快捷键"
}

pub fn translate_help_enter() -> &'static str {
    "下一步"
}

pub fn translate_help_esc() -> &'static str {
    "上一步"
}

pub fn translate_help_jump() -> &'static str {
    "跳转到步骤"
}

pub fn translate_help_navigate() -> &'static str {
    "导航列表"
}

pub fn translate_help_toggle() -> &'static str {
    "切换选中"
}

pub fn translate_help_start_scan() -> &'static str {
    "开始扫描"
}

pub fn translate_help_select_all() -> &'static str {
    "全选"
}

pub fn translate_help_select_none() -> &'static str {
    "全不选"
}

pub fn translate_help_default_selection() -> &'static str {
    "默认选中"
}

pub fn translate_help_adjust_settings() -> &'static str {
    "调整设置值"
}

pub fn translate_help_search() -> &'static str {
    "搜索过滤（项目）"
}

pub fn translate_help_quit() -> &'static str {
    "退出"
}

pub fn translate_help_toggle_help() -> &'static str {
    "切换此帮助"
}

pub fn translate_help_close_hint() -> &'static str {
    "按 ? 或 Esc 关闭"
}

pub fn translate_confirm_title() -> &'static str {
    "确认清理"
}

pub fn translate_confirm_dry_run_title() -> &'static str {
    "确认干跑"
}

pub fn translate_confirm_question() -> &'static str {
    "确定要继续清理吗？"
}

pub fn translate_confirm_dry_run_warning() -> &'static str {
    "干跑模式：不会实际删除任何文件。"
}

pub fn translate_confirm_warning() -> &'static str {
    "警告：此操作不可撤销！"
}

pub fn translate_confirm_confirm() -> &'static str {
    "确认"
}

pub fn translate_confirm_cancel() -> &'static str {
    "取消"
}

pub fn translate_status_scanning() -> &'static str {
    "正在扫描..."
}

pub fn translate_status_dry_run() -> &'static str {
    "干跑进行中（不会删除任何文件）..."
}

pub fn translate_status_cleaning() -> &'static str {
    "清理进行中..."
}

pub fn translate_status_dashboard() -> &'static str {
    "Enter:下一步  s:重扫  ?:帮助  q:退出"
}

pub fn translate_status_categories() -> &'static str {
    "Esc:返回  Enter:下一步  空格:切换  左右:调整  a/n/d:选择"
}

pub fn translate_status_projects() -> &'static str {
    "Esc:返回  Enter:下一步  空格:切换  /:搜索"
}

pub fn translate_status_preview() -> &'static str {
    "Esc:返回  Enter:执行  ?:帮助"
}

pub fn translate_status_cleaning_done() -> &'static str {
    "s:重扫  q:退出"
}

pub fn translate_format_age_years(days: u64) -> String {
    format!("{}年前", days / 365)
}

pub fn translate_format_age_months(days: u64) -> String {
    format!("{}月前", days / 30)
}

pub fn translate_format_age_days(days: i64) -> String {
    format!("{}天前", days)
}

pub fn translate_format_age_hours(hours: i64) -> String {
    format!("{}小时前", hours)
}

pub fn translate_format_age_just_now() -> String {
    "刚刚".into()
}

pub fn translate_format_expired(expiry_days: u32) -> String {
    format!(">{}天", expiry_days)
}

pub fn translate_format_files(count: usize) -> String {
    format!("{}个文件", count)
}

pub fn translate_format_items(count: usize) -> String {
    format!("{}项", count)
}

pub fn translate_format_expired_files(exp_count: usize, total: usize, expiry_days: u32) -> String {
    format!("{}/{} 个文件（>{}天）", exp_count, total, expiry_days)
}

pub fn translate_format_expired_files_simple(exp_count: usize) -> String {
    format!("{}个文件", exp_count)
}

pub fn translate_format_orphan_active(orphan: usize, active: usize) -> String {
    format!("（{}个孤立 + {}个活跃）", orphan, active)
}

pub fn translate_format_orphan(orphan: usize) -> String {
    format!("（{}个孤立）", orphan)
}

pub fn translate_format_active(active: usize) -> String {
    format!("（{}个活跃）", active)
}

pub fn translate_format_all() -> String {
    "（全部）".into()
}

pub fn translate_format_expired_count(expired_count: usize) -> String {
    format!("{}个过期文件", expired_count)
}

pub fn translate_projects_table_status_orphan() -> &'static str {
    "孤立"
}

pub fn translate_projects_table_status_active() -> &'static str {
    "活跃"
}

pub fn translate_projects_table_reclaimable_all() -> &'static str {
    "（全部）"
}

pub fn translate_projects_table_reclaimable_expired() -> &'static str {
    "（{}个过期）"
}

pub fn translate_projects_summary_projects() -> &'static str {
    "个项目"
}

pub fn translate_projects_summary_orphan() -> &'static str {
    "孤立"
}

pub fn translate_projects_summary_active() -> &'static str {
    "活跃"
}

pub fn translate_projects_summary_selected() -> &'static str {
    "已选中"
}

pub fn translate_preview_config_parts(parts: &[&str]) -> String {
    parts.join("、")
}

pub fn translate_select_no_scan_data() -> &'static str {
    "无扫描数据。请先在扫描页按 [S] 扫描。"
}
