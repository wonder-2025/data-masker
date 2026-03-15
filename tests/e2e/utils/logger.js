const fs = require('fs');
const path = require('path');

class TestLogger {
  constructor() {
    this.logDir = 'test-results/logs';
    this.errorLog = [];
    this.currentTest = null;
    
    // 确保日志目录存在
    if (!fs.existsSync(this.logDir)) {
      fs.mkdirSync(this.logDir, { recursive: true });
    }
  }

  startTest(testName) {
    this.currentTest = {
      name: testName,
      startTime: new Date().toISOString(),
      steps: [],
      errors: []
    };
    this.log(`\n========== 开始测试: ${testName} ==========`);
  }

  log(message, type = 'info') {
    const timestamp = new Date().toISOString();
    const logMessage = `[${timestamp}] [${type.toUpperCase()}] ${message}`;
    console.log(logMessage);
    
    if (this.currentTest) {
      this.currentTest.steps.push({
        timestamp,
        type,
        message
      });
    }
  }

  logError(error, context = '') {
    const errorInfo = {
      timestamp: new Date().toISOString(),
      test: this.currentTest?.name || 'Unknown',
      context,
      error: {
        message: error.message || error,
        stack: error.stack || '',
        name: error.name || 'Error'
      }
    };
    
    this.errorLog.push(errorInfo);
    
    if (this.currentTest) {
      this.currentTest.errors.push(errorInfo);
    }
    
    this.log(`❌ ERROR: ${context} - ${error.message || error}`, 'error');
    
    // 保存到单独的错误文件
    const errorFile = path.join(this.logDir, `error-${Date.now()}.json`);
    fs.writeFileSync(errorFile, JSON.stringify(errorInfo, null, 2));
  }

  endTest(success) {
    if (this.currentTest) {
      this.currentTest.endTime = new Date().toISOString();
      this.currentTest.success = success;
      
      // 保存测试日志
      const safeName = this.currentTest.name.replace(/[^a-zA-Z0-9\u4e00-\u9fa5]/g, '_');
      const logFile = path.join(this.logDir, `${safeName}.json`);
      fs.writeFileSync(logFile, JSON.stringify(this.currentTest, null, 2));
    }
    
    this.log(`========== 测试结束: ${success ? '✅ 通过' : '❌ 失败'} ==========\n`);
  }

  generateReport() {
    const report = {
      timestamp: new Date().toISOString(),
      totalTests: this.errorLog.length === 0 ? 'All passed' : this.errorLog.length,
      errors: this.errorLog,
      summary: {
        totalErrors: this.errorLog.length,
        errorTypes: {}
      }
    };
    
    // 统计错误类型
    this.errorLog.forEach(err => {
      const type = err.error.name;
      report.summary.errorTypes[type] = (report.summary.errorTypes[type] || 0) + 1;
    });
    
    // 保存完整报告
    const reportFile = path.join(this.logDir, 'test-report.json');
    fs.writeFileSync(reportFile, JSON.stringify(report, null, 2));
    
    return report;
  }
}

module.exports = new TestLogger();
