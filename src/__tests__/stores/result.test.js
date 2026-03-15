/**
 * 处理结果 Store 测试
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useResultStore } from '@/stores/result'

describe('Result Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  describe('初始状态', () => {
    it('初始结果列表应为空', () => {
      const store = useResultStore()
      expect(store.results).toEqual([])
    })

    it('初始进度状态应为 idle', () => {
      const store = useResultStore()
      expect(store.progress.status).toBe('idle')
    })

    it('初始日志应为空', () => {
      const store = useResultStore()
      expect(store.logs).toEqual([])
    })

    it('初始映射表应为空', () => {
      const store = useResultStore()
      expect(store.mappingTable).toEqual({})
    })
  })

  describe('结果操作', () => {
    it('addResult 应添加处理结果', () => {
      const store = useResultStore()
      
      store.addResult({
        fileId: 'file-1',
        fileName: 'test.txt',
        status: 'done',
        sensitiveInfo: [
          { type: 'phone', original: '138****5678' }
        ],
        processingTime: '1.5s'
      })
      
      expect(store.results.length).toBe(1)
      expect(store.results[0].fileName).toBe('test.txt')
    })

    it('clearResults 应清空结果', () => {
      const store = useResultStore()
      
      store.addResult({
        fileId: 'file-1',
        fileName: 'test.txt',
        status: 'done',
        sensitiveInfo: []
      })
      
      store.clearResults()
      expect(store.results.length).toBe(0)
      expect(store.progress.status).toBe('idle')
    })
  })

  describe('进度更新', () => {
    it('updateProgress 应更新进度信息', () => {
      const store = useResultStore()
      
      store.updateProgress({
        current: 1,
        total: 5,
        currentFile: 'test.txt',
        status: 'processing'
      })
      
      expect(store.progress.current).toBe(1)
      expect(store.progress.total).toBe(5)
      expect(store.progress.currentFile).toBe('test.txt')
      expect(store.progress.status).toBe('processing')
    })
  })

  describe('日志操作', () => {
    it('addLog 应添加日志条目', () => {
      const store = useResultStore()
      
      store.addLog('info', '开始处理文件')
      
      expect(store.logs.length).toBe(1)
      expect(store.logs[0].level).toBe('info')
      expect(store.logs[0].message).toBe('开始处理文件')
    })

    it('clearLogs 应清空日志', () => {
      const store = useResultStore()
      
      store.addLog('info', '日志1')
      store.addLog('error', '日志2')
      
      store.clearLogs()
      expect(store.logs.length).toBe(0)
    })

    it('日志应限制最大数量', () => {
      const store = useResultStore()
      
      // 添加超过 1000 条日志
      for (let i = 0; i < 1100; i++) {
        store.addLog('info', `日志 ${i}`)
      }
      
      expect(store.logs.length).toBeLessThanOrEqual(1000)
    })
  })

  describe('映射表操作', () => {
    it('setMappingTable 应设置映射表', () => {
      const store = useResultStore()
      
      store.setMappingTable({ '13812345678': '138****5678' })
      
      expect(store.mappingTable['13812345678']).toBe('138****5678')
    })

    it('addMapping 应添加映射项', () => {
      const store = useResultStore()
      
      store.addMapping('original@example.com', 'masked@example.com')
      
      expect(store.mappingTable['original@example.com']).toBe('masked@example.com')
    })

    it('clearMapping 应清空映射表', () => {
      const store = useResultStore()
      
      store.addMapping('key', 'value')
      store.clearMapping()
      
      expect(store.mappingTable).toEqual({})
    })

    it('exportMapping 应返回 JSON 字符串', () => {
      const store = useResultStore()
      
      store.addMapping('key', 'value')
      const exported = store.exportMapping()
      
      expect(typeof exported).toBe('string')
      expect(JSON.parse(exported)).toEqual({ key: 'value' })
    })
  })

  describe('计算属性', () => {
    beforeEach(() => {
      setActivePinia(createPinia())
    })

    it('successCount 应计算成功数量', () => {
      const store = useResultStore()
      
      store.addResult({ fileId: '1', fileName: 'a.txt', status: 'done', sensitiveInfo: [] })
      store.addResult({ fileId: '2', fileName: 'b.txt', status: 'done', sensitiveInfo: [] })
      store.addResult({ fileId: '3', fileName: 'c.txt', status: 'error', sensitiveInfo: [] })
      
      expect(store.successCount).toBe(2)
    })

    it('errorCount 应计算失败数量', () => {
      const store = useResultStore()
      
      store.addResult({ fileId: '1', fileName: 'a.txt', status: 'done', sensitiveInfo: [] })
      store.addResult({ fileId: '2', fileName: 'b.txt', status: 'error', sensitiveInfo: [] })
      store.addResult({ fileId: '3', fileName: 'c.txt', status: 'error', sensitiveInfo: [] })
      
      expect(store.errorCount).toBe(2)
    })

    it('sensitiveStats 应统计敏感信息类型', () => {
      const store = useResultStore()
      
      store.addResult({
        fileId: '1',
        fileName: 'a.txt',
        status: 'done',
        sensitiveInfo: [
          { type: 'phone' },
          { type: 'id_card' },
          { type: 'phone' }
        ]
      })
      
      expect(store.sensitiveStats.phone).toBe(2)
      expect(store.sensitiveStats.id_card).toBe(1)
    })

    it('totalSensitive 应计算总敏感信息数量', () => {
      const store = useResultStore()
      
      store.addResult({
        fileId: '1',
        fileName: 'a.txt',
        status: 'done',
        sensitiveInfo: [
          { type: 'phone' },
          { type: 'id_card' }
        ]
      })
      
      expect(store.totalSensitive).toBe(2)
    })
  })

  describe('报告生成', () => {
    it('getReportData 应返回完整报告数据', () => {
      const store = useResultStore()
      
      store.addResult({
        fileId: '1',
        fileName: 'test.txt',
        status: 'done',
        sensitiveInfo: [{ type: 'phone' }],
        processingTime: '1.5s'
      })
      
      const report = store.getReportData()
      
      expect(report).toHaveProperty('summary')
      expect(report).toHaveProperty('sensitiveStats')
      expect(report).toHaveProperty('results')
      expect(report).toHaveProperty('generatedAt')
      expect(report.summary.totalFiles).toBe(1)
    })
  })
})
