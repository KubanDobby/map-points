<script setup lang="ts">
import { reactive, ref } from 'vue'


interface Point {
  id: number
  lng: number
  lat: number
}

interface CreatePointDto {
  lng: number
  lat: number
}

const API_URL = 'http://localhost:3000'

const loading = ref(false)

const error = ref('')

const points = ref<Point[]>([])

const newPoint = reactive<CreatePointDto>({
  lng: 0,
  lat: 0,
})

async function loadPoints() {
  error.value = ''

  try {
    const response = await fetch(`${API_URL}/points`)

    if (!response.ok) {
      throw new Error('Failed to load points')
    }

    points.value = await response.json()
  } catch (err) {
    error.value =
      err instanceof Error
        ? err.message
        : 'Unknown error'
  }
}

async function createPoint() {
  error.value = ''

  loading.value = true

  try {
    const response = await fetch(`${API_URL}/points`, {
      method: 'POST',

      headers: {
        'Content-Type': 'application/json',
      },

      body: JSON.stringify(newPoint),
    })

    if (!response.ok) {
      const text = await response.text()

      throw new Error(text)
    }

    const point: Point = await response.json()

    points.value.unshift(point)

    newPoint.lng = 0
    newPoint.lat = 0
  } catch (err) {
    error.value =
      err instanceof Error
        ? err.message
        : 'Unknown error'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div>Map Points</div>
</template>

<style scoped>

</style>