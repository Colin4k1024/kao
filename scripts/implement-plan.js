/**
 * 执行计划
 * 根据计划文件执行 AI Agent 任务
 */

const fs = require('fs');
const path = require('path');

/**
 * 执行单个任务
 * @param {Object} task - 任务对象
 */
async function executeTask(task) {
  console.log(`🔄 执行任务: ${task.id} - ${task.description}`);

  // 模拟 Agent 执行
  // 实际场景中，这里会调用 Claude Code / Codex API
  await new Promise(resolve => setTimeout(resolve, 1000));

  task.status = 'completed';
  task.completedAt = new Date().toISOString();

  console.log(`✅ 任务完成: ${task.id}`);
  return task;
}

/**
 * 执行完整计划
 * @param {string} planPath - 计划文件路径
 */
async function implementPlan(planPath) {
  if (!planPath) {
    console.error('❌ 请提供计划文件路径');
    process.exit(1);
  }

  const plan = JSON.parse(fs.readFileSync(planPath, 'utf-8'));

  console.log(`📋 开始执行计划: ${plan.requirement}`);
  console.log(`📊 任务总数: ${plan.tasks.length}\n`);

  for (const task of plan.tasks) {
    // 检查依赖
    const deps = plan.dependencies.filter(d => d.taskId === task.id);
    for (const dep of deps) {
      const depTask = plan.tasks.find(t => t.id === dep.dependsOn);
      if (depTask && depTask.status !== 'completed') {
        console.log(`⏳ 等待依赖任务: ${dep.dependsOn}`);
        await executeTask(depTask);
      }
    }

    // 执行任务
    await executeTask(task);
  }

  console.log('\n✅ 所有任务完成!');

  // 更新计划状态
  plan.status = 'completed';
  plan.completedAt = new Date().toISOString();
  fs.writeFileSync(planPath, JSON.stringify(plan, null, 2));
}

// CLI 入口
if (require.main === module) {
  const planPath = process.argv[2];
  implementPlan(planPath).catch(console.error);
}

module.exports = { implementPlan, executeTask };
