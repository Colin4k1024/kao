/**
 * 生成执行计划
 * 根据用户需求生成 AI Agent 可执行的任务计划
 */

const fs = require('fs');
const path = require('path');

/**
 * 解析自然语言需求
 * @param {string} requirement - 用户需求描述
 * @returns {Object} 执行计划
 */
function parseRequirement(requirement) {
  const plan = {
    version: '1.0',
    createdAt: new Date().toISOString(),
    requirement,
    tasks: [],
    dependencies: [],
  };

  // 关键词检测
  const lower = requirement.toLowerCase();

  // 前端任务
  if (lower.includes('页面') || lower.includes('组件') || lower.includes('前端') || lower.includes('ui')) {
    plan.tasks.push({
      id: 'frontend-1',
      type: 'frontend',
      agent: 'Frontend Agent',
      description: '生成前端页面/组件',
      status: 'pending',
    });
  }

  // 后端任务
  if (lower.includes('api') || lower.includes('接口') || lower.includes('后端') || lower.includes('服务')) {
    plan.tasks.push({
      id: 'backend-1',
      type: 'backend',
      agent: 'Backend Agent',
      description: '开发 API 端点',
      status: 'pending',
    });
  }

  // 数据库任务
  if (lower.includes('数据库') || lower.includes('schema') || lower.includes('模型')) {
    plan.tasks.push({
      id: 'database-1',
      type: 'database',
      agent: 'Database Agent',
      description: '设计/更新数据库 Schema',
      status: 'pending',
    });
  }

  return plan;
}

/**
 * 生成执行计划
 * @param {string} requirement - 用户需求
 */
function createPlan(requirement) {
  const plan = parseRequirement(requirement);

  // 保存计划文件
  const planPath = path.join(__dirname, '..', '.agent', 'plans', `${Date.now()}.json`);
  fs.mkdirSync(path.dirname(planPath), { recursive: true });
  fs.writeFileSync(planPath, JSON.stringify(plan, null, 2));

  console.log(`✅ 执行计划已生成: ${planPath}`);
  console.log(`📋 任务数量: ${plan.tasks.length}`);

  return plan;
}

// CLI 入口
if (require.main === module) {
  const requirement = process.argv.slice(2).join(' ');
  if (!requirement) {
    console.error('❌ 请提供需求描述');
    console.log('用法: node create-plan.js "开发用户注册功能"');
    process.exit(1);
  }
  createPlan(requirement);
}

module.exports = { createPlan, parseRequirement };
