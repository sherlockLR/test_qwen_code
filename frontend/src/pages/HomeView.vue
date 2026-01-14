<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <h1 class="text-4xl font-extrabold text-gray-900 sm:text-5xl sm:tracking-tight lg:text-6xl">
        传记写作助手
      </h1>
      <p class="mt-6 max-w-lg mx-auto text-xl text-gray-500">
        用AI的力量，记录生命中的每一个精彩瞬间
      </p>
    </div>

    <div class="mt-16 grid gap-8 lg:grid-cols-3 lg:max-w-none">
      <div class="flex flex-col rounded-2xl bg-white p-8 shadow-xl">
        <div class="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
        </div>
        <h3 class="mt-6 text-lg font-medium text-gray-900">智能写作建议</h3>
        <p class="mt-4 flex-1 text-gray-500">
          AI将为您提供写作建议和内容优化，让您的传记更加生动有趣。
        </p>
      </div>

      <div class="flex flex-col rounded-2xl bg-white p-8 shadow-xl">
        <div class="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
        </div>
        <h3 class="mt-6 text-lg font-medium text-gray-900">自动生成大纲</h3>
        <p class="mt-4 flex-1 text-gray-500">
          AI可以为您自动生成传记大纲，帮助您构建清晰的写作框架。
        </p>
      </div>

      <div class="flex flex-col rounded-2xl bg-white p-8 shadow-xl">
        <div class="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
          </svg>
        </div>
        <h3 class="mt-6 text-lg font-medium text-gray-900">智能采访提示</h3>
        <p class="mt-4 flex-1 text-gray-500">
          AI将提供引导式提问，帮助您系统地收集人生故事和经历。
        </p>
      </div>
    </div>

    <div class="mt-16 text-center">
      <router-link 
        to="/dashboard" 
        class="inline-block px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700"
      >
        开始写作
      </router-link>
    </div>
  </div>
</template>

<script>
export default {
  name: 'HomeView'
}
</script>