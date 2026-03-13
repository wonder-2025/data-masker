// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <el-container class="app-container">
    <!-- 左侧导航栏 -->
    <el-aside width="220px" class="app-aside">
      <div class="logo">
        <el-icon :size="32" color="#409EFF"><Shield /></el-icon>
        <span class="logo-text">Data Masker</span>
      </div>
      
      <el-menu
        :default-active="activeMenu"
        class="app-menu"
        router
        background-color="#1a1a2e"
        text-color="#a0a0a0"
        active-text-color="#409EFF"
      >
        <el-menu-item index="/">
          <el-icon><HomeFilled /></el-icon>
          <span>首页</span>
        </el-menu-item>
        
        <el-menu-item index="/file-select">
          <el-icon><FolderOpened /></el-icon>
          <span>文件选择</span>
        </el-menu-item>
        
        <el-menu-item index="/rule-config">
          <el-icon><Setting /></el-icon>
          <span>规则配置</span>
        </el-menu-item>
        
        <el-menu-item index="/preview">
          <el-icon><View /></el-icon>
          <span>预览确认</span>
        </el-menu-item>
        
        <el-menu-item index="/result">
          <el-icon><CircleCheck /></el-icon>
          <span>处理结果</span>
        </el-menu-item>
        
        <el-divider />
        
        <el-menu-item index="/settings">
          <el-icon><Tools /></el-icon>
          <span>设置</span>
        </el-menu-item>
      </el-menu>
      
      <div class="app-footer">
        <span>v1.0.0</span>
        <el-divider direction="vertical" />
        <span>本地处理</span>
      </div>
    </el-aside>
    
    <!-- 主内容区 -->
    <el-main class="app-main">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </el-main>
  </el-container>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const activeMenu = computed(() => route.path)
</script>

<style lang="scss" scoped>
.app-container {
  height: 100vh;
  background: #f5f7fa;
}

.app-aside {
  background: #1a1a2e;
  display: flex;
  flex-direction: column;
  
  .logo {
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    border-bottom: 1px solid #2a2a3e;
    
    .logo-text {
      font-size: 20px;
      font-weight: 600;
      color: #fff;
      letter-spacing: 1px;
    }
  }
  
  .app-menu {
    border-right: none;
    flex: 1;
    
    .el-menu-item {
      height: 50px;
      line-height: 50px;
      margin: 4px 8px;
      border-radius: 8px;
      
      &:hover {
        background: #2a2a3e !important;
      }
      
      &.is-active {
        background: linear-gradient(135deg, #409EFF 0%, #36D1DC 100%) !important;
        color: #fff !important;
      }
    }
  }
  
  .app-footer {
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #666;
    font-size: 12px;
    border-top: 1px solid #2a2a3e;
  }
}

.app-main {
  padding: 24px;
  overflow-y: auto;
}

// 页面切换动画
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
