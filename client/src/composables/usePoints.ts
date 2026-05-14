import { reactive, ref } from 'vue'


interface Point {
  id: number
  lng: number
  lat: number
  camp: 'friend' | 'enemy'
}

interface CreatePointDto {
  lng: number
  lat: number
  camp: 'friend' | 'enemy'
}

const API_URL = 'http://localhost:3000'

export const usePoints = () => {
  const loading = ref(false)

  const error = ref('')

  const points = ref<Point[]>([])

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

  async function createPoint(newPoint: CreatePointDto) {
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
    } catch (err) {
      error.value =
        err instanceof Error
          ? err.message
          : 'Unknown error'
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    error,
    points,
    loadPoints,
    createPoint,
  }
}