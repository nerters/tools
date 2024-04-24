import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
// import Layout from '../components/HelloWorld.vue'

const routes: Array<RouteRecordRaw> = [
  {
  //路由初始指向
    path: '/',
    name: 'Index',
    component:()=>import('../index/Index.vue')
  },
  {
    //路由初始指向
      path: '/main',
      component:()=>import('../pages/Index.vue'),
      children:[
        {
        //路由初始指向
          path: 'Json',
          name: 'Json',
          meta: {
            title: 'JSON'
          },
          component:()=>import('../pages/Json.vue')
        },
        {
          //路由初始指向
            path: 'Merge',
            name: 'Merge',
            meta: {
              title: 'Merge'
            },
            component:()=>import('../pages/Merge.vue')
          }
          ,
        {
          //路由初始指向
            path: 'Img',
            name: 'Img',
            meta: {
              title: 'Img'
            },
            component:()=>import('../pages/Img.vue')
          },
          {
            //路由初始指向
              path: 'RsaPage',
              name: 'RsaPage',
              meta: {
                title: 'RsaPage'
              },
              component:()=>import('../pages/RsaPage.vue')
          },
          {
            //路由初始指向
              path: 'Time',
              name: 'Time',
              meta: {
                title: 'Time'
              },
              component:()=>import('../pages/Time.vue')
          },
          {
            //路由初始指向
              path: 'CronTitle',
              name: 'CronTitle',
              meta: {
                title: 'CronTitle'
              },
              component:()=>import('../pages/CronTitle.vue')
          }
        ,
        {
          path: '/:catchAll(.*)',
          name: 'default',
          meta: {
            title: 'default'
          },
          component:()=>import('../pages/default.vue')
        }
      ]
  },
  {
      //路由初始指向
      path: '/hint',
      name: 'hint',
      component:()=>import('../hint/Index.vue'),
      children:[
        {
          //路由初始指向
            path: 'Cron',
            name: 'Cron',
            meta: {
              title: 'Cron'
            },
            component:()=>import('../hint/Cron.vue')
        }
      ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
