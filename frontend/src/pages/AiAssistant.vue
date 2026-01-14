<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <h1 class="text-3xl font-extrabold text-gray-900 sm:text-4xl">
        AI传记助手
      </h1>
      <p class="mt-3 max-w-2xl mx-auto text-xl text-gray-500 sm:mt-4">
        让AI帮助您更好地记录人生故事
      </p>
    </div>

    <div class="mt-10">
      <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">生成大纲</h3>
            <div class="mt-2 max-w-xl text-sm text-gray-500">
              <p>AI将根据您提供的信息自动生成传记大纲。</p>
            </div>
            <div class="mt-5">
              <button
                @click="generateOutline"
                type="button"
                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                开始生成
              </button>
            </div>
          </div>
        </div>

        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">采访问题</h3>
            <div class="mt-2 max-w-xl text-sm text-gray-500">
              <p>AI将生成一系列引导性问题，帮助您收集人生故事。</p>
            </div>
            <div class="mt-5">
              <button
                @click="getInterviewQuestions"
                type="button"
                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                生成问题
              </button>
            </div>
          </div>
        </div>

        <div class="bg-white overflow-hidden shadow rounded-lg">
          <div class="px-4 py-5 sm:p-6">
            <h3 class="text-lg leading-6 font-medium text-gray-900">内容优化</h3>
            <div class="mt-2 max-w-xl text-sm text-gray-500">
              <p>AI将帮助您优化和完善传记内容。</p>
            </div>
            <div class="mt-5">
              <button
                @click="optimizeContent"
                type="button"
                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                优化内容
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 结果显示区域 -->
    <div class="mt-10">
      <div class="bg-white shadow overflow-hidden sm:rounded-lg">
        <div class="px-4 py-5 sm:px-6">
          <h3 class="text-lg leading-6 font-medium text-gray-900">AI结果</h3>
          <p class="mt-1 max-w-2xl text-sm text-gray-500">
            AI助手生成的结果将在此处显示
          </p>
        </div>
        <div class="border-t border-gray-200 px-4 py-5 sm:p-0">
          <div class="py-5 sm:px-6">
            <div v-if="result" class="bg-gray-50 rounded-md p-4">
              <pre class="text-sm text-gray-800 whitespace-pre-wrap">{{ result }}</pre>
            </div>
            <div v-else class="text-center text-gray-500 py-8">
              <p>等待AI助手生成结果...</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import axios from 'axios'

export default {
  name: 'AiAssistant',
  setup() {
    const result = ref('')

    const generateOutline = async () => {
      try {
        result.value = '正在生成大纲...'
        const response = await axios.post('/api/ai/generate-outline', {})
        if (response.data.success) {
          const outlineData = JSON.parse(response.data.data)
          result.value = `传记大纲生成成功！\n\n${JSON.stringify(outlineData, null, 2)}`
        }
      } catch (error) {
        console.error('生成大纲失败:', error)
        result.value = '生成大纲失败，请重试'
      }
    }

    const getInterviewQuestions = async () => {
      try {
        result.value = '正在生成采访问题...'
        const response = await axios.post('/api/ai/interview-questions', {})
        if (response.data.success) {
          const questions = JSON.parse(response.data.data)
          result.value = `采访问题生成成功！\n\n${questions.join('\n')}`
        }
      } catch (error) {
        console.error('生成采访问题失败:', error)
        result.value = '生成采访问题失败，请重试'
      }
    }

    const optimizeContent = async () => {
      try {
        result.value = '正在优化内容...'
        // 模拟优化内容功能
        setTimeout(() => {
          result.value = '内容优化功能将在后续版本中实现。此功能需要接收用户输入的内容，然后调用AI API进行优化。'
        }, 1000)
      } catch (error) {
        console.error('优化内容失败:', error)
        result.value = '优化内容失败，请重试'
      }
    }

    return {
      result,
      generateOutline,
      getInterviewQuestions,
      optimizeContent
    }
  }
}
</script>