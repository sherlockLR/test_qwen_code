<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-xl font-semibold text-gray-900">我的传记项目</h1>
        <p class="mt-2 text-sm text-gray-700">
          管理您的传记项目，开始记录珍贵的人生故事。
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button
          @click="createNewBiography"
          type="button"
          class="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
        >
          新建传记
        </button>
      </div>
    </div>

    <div class="mt-8 flex flex-col">
      <div class="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
        <div class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
          <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
            <table class="min-w-full divide-y divide-gray-300">
              <thead class="bg-gray-50">
                <tr>
                  <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
                    标题
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    描述
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    状态
                  </th>
                  <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    创建时间
                  </th>
                  <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
                    <span class="sr-only">编辑</span>
                  </th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200 bg-white">
                <tr v-for="biography in biographies" :key="biography.id">
                  <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                    {{ biography.title }}
                  </td>
                  <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                    {{ biography.description || '暂无描述' }}
                  </td>
                  <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                    <span 
                      class="inline-flex rounded-full bg-green-100 px-2 text-xs font-semibold leading-5 text-green-800"
                      v-if="biography.status === 'Published'"
                    >
                      已发布
                    </span>
                    <span 
                      class="inline-flex rounded-full bg-yellow-100 px-2 text-xs font-semibold leading-5 text-yellow-800"
                      v-else-if="biography.status === 'Draft'"
                    >
                      草稿
                    </span>
                    <span 
                      class="inline-flex rounded-full bg-gray-100 px-2 text-xs font-semibold leading-5 text-gray-800"
                      v-else
                    >
                      已归档
                    </span>
                  </td>
                  <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                    {{ formatDate(biography.created_at) }}
                  </td>
                  <td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                    <router-link 
                      :to="`/editor/${biography.id}`" 
                      class="text-indigo-600 hover:text-indigo-900 mr-4"
                    >
                      编辑
                    </router-link>
                    <button 
                      @click="deleteBiography(biography.id)"
                      class="text-red-600 hover:text-red-900"
                    >
                      删除
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

export default {
  name: 'DashboardView',
  setup() {
    const biographies = ref([])
    const router = useRouter()
    const user = JSON.parse(localStorage.getItem('user') || 'null')

    if (!user) {
      router.push('/')
      return
    }

    const fetchBiographies = async () => {
      try {
        const response = await axios.get(`/api/biographies?user_id=${user.id}`)
        if (response.data.success) {
          biographies.value = response.data.data
        }
      } catch (error) {
        console.error('获取传记列表失败:', error)
      }
    }

    const createNewBiography = async () => {
      try {
        const title = prompt('请输入传记标题:')
        if (title) {
          const response = await axios.post('/api/biographies', {
            user_id: user.id,
            title: title,
            description: '新创建的传记项目'
          })
          
          if (response.data.success) {
            biographies.value.unshift(response.data.data)
            router.push(`/editor/${response.data.data.id}`)
          }
        }
      } catch (error) {
        console.error('创建传记失败:', error)
      }
    }

    const deleteBiography = async (id) => {
      if (confirm('确定要删除这个传记项目吗？')) {
        try {
          await axios.delete(`/api/biographies/${id}`)
          biographies.value = biographies.value.filter(b => b.id !== id)
        } catch (error) {
          console.error('删除传记失败:', error)
        }
      }
    }

    const formatDate = (dateString) => {
      return new Date(dateString).toLocaleDateString('zh-CN')
    }

    onMounted(() => {
      fetchBiographies()
    })

    return {
      biographies,
      createNewBiography,
      deleteBiography,
      formatDate
    }
  }
}
</script>