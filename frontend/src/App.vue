<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0 flex items-center">
              <span class="text-xl font-bold text-indigo-600">传记写作助手</span>
            </div>
          </div>
          <div class="flex items-center">
            <button 
              v-if="!user"
              @click="login"
              class="ml-4 px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            >
              登录
            </button>
            <div v-else class="flex items-center">
              <img :src="user.avatar || '/src/assets/default-avatar.png'" alt="Avatar" class="w-8 h-8 rounded-full">
              <span class="ml-2 text-gray-700">{{ user.nickname }}</span>
            </div>
          </div>
        </div>
      </div>
    </nav>

    <!-- 主要内容区域 -->
    <main class="py-6">
      <router-view />
    </main>

    <!-- 底部 -->
    <footer class="bg-white mt-10">
      <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <p class="text-center text-gray-500 text-sm">
          © {{ new Date().getFullYear() }} 传记写作助手. 保留所有权利.
        </p>
      </div>
    </footer>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

export default {
  name: 'App',
  setup() {
    const user = ref(null)
    const router = useRouter()

    // 模拟微信登录
    const login = async () => {
      try {
        // 在实际实现中，这里会调用微信授权登录
        const response = await axios.post('/api/users', {
          openid: 'mock_openid_' + Math.random().toString(36).substr(2, 9),
          nickname: '测试用户' + Math.floor(Math.random() * 1000),
          avatar: null
        })
        
        if (response.data.success) {
          user.value = response.data.data
          localStorage.setItem('user', JSON.stringify(response.data.data))
          router.push('/dashboard')
        }
      } catch (error) {
        console.error('登录失败:', error)
      }
    }

    onMounted(() => {
      // 尝试从本地存储恢复用户信息
      const storedUser = localStorage.getItem('user')
      if (storedUser) {
        user.value = JSON.parse(storedUser)
      }
    })

    return {
      user,
      login
    }
  }
}
</script>