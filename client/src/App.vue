<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue';
import {YMap, YMapDefaultSchemeLayer, YMapDefaultFeaturesLayer, YMapMarker, YMapListener} from './lib/ymaps';
import type {YMapLocationRequest} from 'ymaps3';
import { usePoints } from './composables/usePoints';

const isCreating = ref(false);
const typeCreating = ref<'friend' | 'enemy'>('friend');

const LOCATION: YMapLocationRequest = {
  center: [37.588144, 55.733842],
  zoom: 2
};

const {
  loadPoints,
  createPoint,
  points,
} = usePoints();

onMounted(() => {
  loadPoints();
});

const addPoint = async (lng: number, lat: number) => {
  await createPoint({lng, lat, camp: typeCreating.value});
  await loadPoints();
}

function onMapCLick(_obj: any, e: {
  coordinates: [number, number]
}) {
  if (!isCreating.value) {
    return;
  }
  const { coordinates } = e;
  addPoint(coordinates[0], coordinates[1]);
  isCreating.value = false;
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape' && isCreating.value) {
    isCreating.value = false;
  }
}

window.addEventListener('keydown', onKeyDown);

onUnmounted(() => {
  window.removeEventListener('keydown', onKeyDown);
});
</script>

<template>
  <div style="width: 100vw; height: 100vh" :style="{cursor: isCreating ? 'crosshair' : 'default'}">
    <div class="point-control">
      <button style="width: 100%;" @click="isCreating = !isCreating">{{ isCreating ? 'Отмена [Esc]' : 'Добавить точку' }}</button>
      <div class="type-control" v-if="isCreating">
        <label>
          <input type="radio" value="friend" v-model="typeCreating" />
          Friendly
        </label>
        <label style="margin-left: 10px;">
          <input type="radio" value="enemy" v-model="typeCreating" />
          Enemy
        </label>
      </div>
      <p style="max-width: 100%; padding: 10px 0 0; margin: 0px;" v-if="isCreating">
        Выберите место на карте для добавления новой точки.
      </p style="max-width: 100%;">
    </div>
    <YMap :location="LOCATION">
      <YMapDefaultSchemeLayer />
      <YMapDefaultFeaturesLayer />
      <YMapListener :onClick="onMapCLick" />
      
      <YMapMarker v-for="point in points" :key="point.id" :coordinates="[point.lng, point.lat]">
        <div style="width: 16px; height: 16px; border-radius: 50%; transform: translate(-50%, -50%);" :style="{background: point.camp === 'friend' ? 'green' : 'red'}"></div>
      </YMapMarker>
    </YMap>
  </div>
</template>

<style>
body {
  margin: 0;
}

.point-control {
  position: absolute;
  z-index: 1000;
  top: 10px;
  right: 10px;
  background: white;
  padding: 10px;
  border-radius: 4px;
  width: 200px;
}

.type-control {
  display: flex;
  margin-top: 10px;
}

.type-control label {
  display: flex;
  align-items: center;
  font-size: 14px;
  font-weight: 500;
}
</style>