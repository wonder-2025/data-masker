/**
 * 文件管理 Store 测试
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useFilesStore } from '@/stores/files'

// Mock localStorage
const localStorageMock = (() => {
  let store = {}
  return {
    getItem: vi.fn((key) => store[key] || null),
    setItem: vi.fn((key, value) => {
      store[key] = value
    }),
    removeItem: vi.fn((key) => {
      delete store[key]
    }),
    clear: vi.fn(() => {
      store = {}
    })
  }
})()

Object.defineProperty(global, 'localStorage', {
  value: localStorageMock
})

describe('Files Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorageMock.clear()
  })

  describe('初始状态', () => {
    it('初始文件列表应为空', () => {
      const store = useFilesStore()
      expect(store.files).toEqual([])
    })

    it('fileCount 应为 0', () => {
      const store = useFilesStore()
      expect(store.fileCount).toBe(0)
    })

    it('totalSize 应为 0', () => {
      const store = useFilesStore()
      expect(store.totalSize).toBe(0)
    })
  })

  describe('文件操作', () => {
    it('addFile 应添加文件', () => {
      const store = useFilesStore()
      
      store.addFile({
        name: 'test.txt',
        path: '/tmp/test.txt',
        size: 1024,
        type: 'text/plain'
      })
      
      expect(store.files.length).toBe(1)
      expect(store.files[0].name).toBe('test.txt')
      expect(store.files[0].status).toBe('pending')
    })

    it('addFile 不应添加重复文件', () => {
      const store = useFilesStore()
      
      store.addFile({
        name: 'test.txt',
        path: '/tmp/test.txt',
        size: 1024,
        type: 'text/plain'
      })
      
      store.addFile({
        name: 'test.txt',
        path: '/tmp/test.txt',
        size: 2048,
        type: 'text/plain'
      })
      
      expect(store.files.length).toBe(1)
    })

    it('addFiles 应批量添加文件', () => {
      const store = useFilesStore()
      
      store.addFiles([
        { name: 'file1.txt', path: '/tmp/file1.txt', size: 1024, type: 'text/plain' },
        { name: 'file2.txt', path: '/tmp/file2.txt', size: 2048, type: 'text/plain' }
      ])
      
      expect(store.files.length).toBe(2)
    })

    it('removeFile 应删除指定文件', () => {
      const store = useFilesStore()
      
      store.addFile({
        id: 'file-1',
        name: 'test.txt',
        path: '/tmp/test.txt',
        size: 1024,
        type: 'text/plain'
      })
      
      store.removeFile('file-1')
      expect(store.files.length).toBe(0)
    })

    it('clearFiles 应清空所有文件', () => {
      const store = useFilesStore()
      
      store.addFiles([
        { name: 'file1.txt', path: '/tmp/file1.txt', size: 1024, type: 'text/plain' },
        { name: 'file2.txt', path: '/tmp/file2.txt', size: 2048, type: 'text/plain' }
      ])
      
      store.clearFiles()
      expect(store.files.length).toBe(0)
    })
  })

  describe('文件状态更新', () => {
    it('updateFileStatus 应更新文件状态', () => {
      const store = useFilesStore()
      
      store.addFile({
        id: 'file-1',
        name: 'test.txt',
        path: '/tmp/test.txt',
        size: 1024,
        type: 'text/plain'
      })
      
      store.updateFileStatus('file-1', 'processing')
      expect(store.files[0].status).toBe('processing')
    })

    it('updateFileResult 应更新文件处理结果', () => {
      const store = useFilesStore()
      
      store.addFile({
        id: 'file-1',
        name: 'test.txt',
        path: '/tmp/test.txt',
        size: 1024,
        type: 'text/plain'
      })
      
      store.updateFileResult('file-1', {
        status: 'done',
        sensitiveCount: 5,
        processingTime: '2.5s',
        outputPath: '/tmp/output/test.txt'
      })
      
      expect(store.files[0].status).toBe('done')
      expect(store.files[0].sensitiveCount).toBe(5)
      expect(store.files[0].processingTime).toBe('2.5s')
    })
  })

  describe('计算属性', () => {
    it('totalSize 应计算正确总大小', () => {
      const store = useFilesStore()
      
      store.addFiles([
        { name: 'file1.txt', path: '/tmp/file1.txt', size: 1024, type: 'text/plain' },
        { name: 'file2.txt', path: '/tmp/file2.txt', size: 2048, type: 'text/plain' }
      ])
      
      expect(store.totalSize).toBe(3072)
    })

    it('fileCount 应返回正确文件数量', () => {
      const store = useFilesStore()
      
      store.addFiles([
        { name: 'file1.txt', path: '/tmp/file1.txt', size: 1024, type: 'text/plain' },
        { name: 'file2.txt', path: '/tmp/file2.txt', size: 2048, type: 'text/plain' }
      ])
      
      expect(store.fileCount).toBe(2)
    })
  })

  describe('最近文件', () => {
    it('addRecentFile 应添加到最近文件', () => {
      const store = useFilesStore()
      
      store.addRecentFile({
        name: 'recent.txt',
        path: '/tmp/recent.txt',
        type: 'text/plain'
      })
      
      expect(store.recentFiles.length).toBe(1)
    })

    it('clearRecentFiles 应清空最近文件', () => {
      const store = useFilesStore()
      
      store.addRecentFile({
        name: 'recent.txt',
        path: '/tmp/recent.txt',
        type: 'text/plain'
      })
      
      store.clearRecentFiles()
      expect(store.recentFiles.length).toBe(0)
    })
  })
})
