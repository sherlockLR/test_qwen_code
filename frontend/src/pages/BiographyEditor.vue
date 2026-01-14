<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="md:grid md:grid-cols-3 md:gap-6">
      <div class="md:col-span-1">
        <div class="sticky top-6">
          <div class="shadow sm:rounded-md">
            <div class="space-y-6 bg-white py-6 px-4 sm:p-6">
              <div>
                <h3 class="text-lg font-medium leading-6 text-gray-900">传记信息</h3>
                <p class="mt-1 text-sm text-gray-500">
                  管理您的传记项目基本信息。
                </p>
              </div>

              <div class="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                <div class="sm:col-span-6">
                  <label for="title" class="block text-sm font-medium text-gray-700">标题</label>
                  <div class="mt-1">
                    <input
                      type="text"
                      v-model="biography.title"
                      name="title"
                      id="title"
                      class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    />
                  </div>
                </div>

                <div class="sm:col-span-6">
                  <label for="description" class="block text-sm font-medium text-gray-700">描述</label>
                  <div class="mt-1">
                    <textarea
                      id="description"
                      name="description"
                      rows="3"
                      v-model="biography.description"
                      class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    />
                  </div>
                </div>

                <div class="sm:col-span-6">
                  <label class="block text-sm font-medium text-gray-700">状态</label>
                  <div class="mt-2 space-y-4">
                    <div class="flex items-center">
                      <input
                        id="draft"
                        name="status"
                        type="radio"
                        value="Draft"
                        v-model="biography.status"
                        class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500"
                      />
                      <label for="draft" class="ml-3 block text-sm font-medium text-gray-700">草稿</label>
                    </div>
                    <div class="flex items-center">
                      <input
                        id="published"
                        name="status"
                        type="radio"
                        value="Published"
                        v-model="biography.status"
                        class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500"
                      />
                      <label for="published" class="ml-3 block text-sm font-medium text-gray-700">已发布</label>
                    </div>
                  </div>
                </div>
              </div>

              <div class="flex justify-end">
                <button
                  @click="saveBiography"
                  type="button"
                  class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                >
                  保存
                </button>
              </div>
            </div>
          </div>

          <div class="mt-6 shadow sm:rounded-md">
            <div class="bg-white py-6 px-4 sm:p-6">
              <div>
                <h3 class="text-lg font-medium leading-6 text-gray-900">AI助手</h3>
                <p class="mt-1 text-sm text-gray-500">
                  使用AI功能增强您的传记写作体验。
                </p>
              </div>

              <div class="mt-6 space-y-4">
                <button
                  @click="generateOutline"
                  type="button"
                  class="w-full inline-flex justify-center items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-purple-600 hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500"
                >
                  生成大纲
                </button>
                
                <button
                  @click="getInterviewQuestions"
                  type="button"
                  class="w-full inline-flex justify-center items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-teal-600 hover:bg-teal-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-500"
                >
                  采访问题
                </button>
                
                <button
                  @click="generateContent"
                  type="button"
                  class="w-full inline-flex justify-center items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  AI续写
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="mt-5 md:col-span-2 md:mt-0">
        <div class="shadow sm:rounded-md">
          <div class="bg-white py-6 px-4 sm:p-6">
            <div>
              <h3 class="text-lg font-medium leading-6 text-gray-900">传记内容</h3>
              <p class="mt-1 text-sm text-gray-500">
                在这里编写和编辑您的传记内容。
              </p>
            </div>

            <div class="mt-6">
              <textarea
                v-model="biography.content"
                rows="20"
                class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
              />
            </div>

            <div class="mt-6 flex justify-end">
              <button
                @click="saveBiography"
                type="button"
                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                保存内容
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'

export default {
  name: 'BiographyEditor',
  setup() {
    const route = useRoute()
    const biographyId = route.params.id
    const biography = ref({
      id: '',
      user_id: '',
      title: '',
      description: '',
      content: '',
      status: 'Draft',
      created_at: '',
      updated_at: ''
    })

    const user = JSON.parse(localStorage.getItem('user') || 'null')

    if (!user) {
      // 在实际应用中应该重定向到登录页面
      console.error('用户未登录')
    }

    const fetchBiography = async () => {
      if (!biographyId) return
      
      try {
        const response = await axios.get(`/api/biographies/${biographyId}`)
        if (response.data.success) {
          biography.value = response.data.data
        }
      } catch (error) {
        console.error('获取传记失败:', error)
      }
    }

    const saveBiography = async () => {
      try {
        const payload = {
          title: biography.value.title,
          description: biography.value.description,
          content: biography.value.content
        }
        
        const response = await axios.post(`/api/biographies/${biography.value.id}`, payload)
        if (response.data.success) {
          alert('保存成功！')
        }
      } catch (error) {
        console.error('保存传记失败:', error)
        alert('保存失败，请重试')
      }
    }

    const generateOutline = async () => {
      try {
        const response = await axios.post('/api/ai/generate-outline', {})
        if (response.data.success) {
          const outlineData = JSON.parse(response.data.data)
          alert('大纲生成成功，内容已复制到剪贴板！')
          console.log('生成的大纲:', outlineData)
        }
      } catch (error) {
        console.error('生成大纲失败:', error)
        alert('生成大纲失败，请重试')
      }
    }

    const generateContent = async () => {
      try {
        const response = await axios.post('/api/ai/generate-content', {})
        if (response.data.success) {
          const content = response.data.data
          biography.value.content += '\n\n' + content
          alert('AI内容生成成功！')
        }
      } catch (error) {
        console.error('生成内容失败:', error)
        alert('生成内容失败，请重试')
      }
    }

    const getInterviewQuestions = async () => {
      try {
        const response = await axios.post('/api/ai/interview-questions', {})
        if (response.data.success) {
          const questions = JSON.parse(response.data.data)
          alert(`获取采访问题成功！示例问题：${questions[0]}`)
          console.log('采访问题:', questions)
        }
      } catch (error) {
        console.error('获取采访问题失败:', error)
        alert('获取采访问题失败，请重试')
      }
    }

    onMounted(() => {
      if (biographyId) {
        fetchBiography()
      }
    })

    return {
      biography,
      saveBiography,
      generateOutline,
      generateContent,
      getInterviewQuestions
    }
  }
}
</script>