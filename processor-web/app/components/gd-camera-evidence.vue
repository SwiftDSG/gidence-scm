<template>
  <div class="gd-evidence">
    <img
      class="gd-evidence-background"
      :src="evidence ? `${api}/frame/${camera_id}?id=${evidence.id}` : ''"
    />
    <img
      class="gd-evidence-image"
      :src="evidence ? `${api}/frame/${camera_id}?id=${evidence.id}` : ''"
    />
    <div class="gd-evidence-overlay">
      <div v-for="person in evidence.person" class="gd-evidence-overlay-person">
        <div
          class="gd-evidence-overlay-person-box"
          :style="{
            top: `${person.bbox[1] * 100}%`,
            left: `${person.bbox[0] * 100}%`,
            width: `${(person.bbox[2] - person.bbox[0]) * 100}%`,
            height: `${(person.bbox[3] - person.bbox[1]) * 100}%`,
          }"
        ></div>
      </div>
      <div v-for="person in evidence.person" class="gd-evidence-overlay-part">
        <div
          v-for="part in person.part"
          class="gd-evidence-overlay-part-box"
          :class="partColor(part.label, person.violation)"
          :style="{
            top: `${part.bbox[1] * 100}%`,
            left: `${part.bbox[0] * 100}%`,
            width: `${(part.bbox[2] - part.bbox[0]) * 100}%`,
            height: `${(part.bbox[3] - part.bbox[1]) * 100}%`,
          }"
        ></div>
      </div>
      <div
        v-for="person in evidence.person"
        class="gd-evidence-overlay-equipment"
      >
        <div
          v-for="equipment in person.equipment"
          class="gd-evidence-overlay-equipment-box"
          :class="equipmentColor(equipment.label, person.violation)"
          :style="{
            top: `${equipment.bbox[1] * 100}%`,
            left: `${equipment.bbox[0] * 100}%`,
            width: `${(equipment.bbox[2] - equipment.bbox[0]) * 100}%`,
            height: `${(equipment.bbox[3] - equipment.bbox[1]) * 100}%`,
          }"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import type {
    Evidence,
    EvidencePersonEquipmentLabel,
    EvidencePersonPartLabel,
    EvidencePersonViolation,
  } from "~/types/evidence";

  const props = defineProps<{
    camera_id: string;
    evidence: Evidence;
  }>();

  const {
    public: { processor: api },
  } = useRuntimeConfig();

  function partColor(
    label: EvidencePersonPartLabel,
    violation: EvidencePersonViolation[],
  ): "--success" | "--error" {
    if (label === "head" && violation.includes("missing_hardhat"))
      return "--error";
    if (
      label === "hand" &&
      (violation.includes("missing_gloves") ||
        violation.includes("improperly_worn_gloves"))
    )
      return "--error";
    if (
      label === "foot" &&
      (violation.includes("missing_shoes") ||
        violation.includes("improperly_worn_shoes"))
    )
      return "--error";
    if (
      label === "face" &&
      (violation.includes("missing_facemask") ||
        violation.includes("improperly_worn_facemask"))
    )
      return "--error";
    if (
      label === "ear" &&
      (violation.includes("missing_earmuffs") ||
        violation.includes("improperly_worn_earmuffs"))
    )
      return "--error";
    return "--success";
  }
  function equipmentColor(
    label: EvidencePersonEquipmentLabel,
    violation: EvidencePersonViolation[],
  ): "--success" | "--error" {
    if (label === "hardhat" && violation.includes("improperly_worn_hardhat"))
      return "--error";
    if (label === "gloves" && violation.includes("improperly_worn_gloves"))
      return "--error";
    if (label === "shoes" && violation.includes("improperly_worn_shoes"))
      return "--error";
    if (label === "facemask" && violation.includes("improperly_worn_facemask"))
      return "--error";
    if (label === "earmuffs" && violation.includes("improperly_worn_earmuffs"))
      return "--error";
    return "--success";
  }
</script>

<style lang="scss" scoped>
  .gd-evidence {
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    &-background {
      position: absolute;
      width: 100%;
      height: 100%;
      object-fit: cover;
      filter: blur(4px);
    }
    &-image {
      position: absolute;
      width: 100%;
      aspect-ratio: 16 / 9;
      object-fit: cover;
    }
    &-overlay {
      position: absolute;
      width: 100%;
      aspect-ratio: 16 / 9;
      &-person {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        &-box {
          position: absolute;
          border: 1px solid #fff;
        }
      }
      &-part {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        &-box {
          position: absolute;
          border: 1px solid #fff;
          &.--success {
            border-color: var(--success-color);
          }
          &.--error {
            border-color: var(--error-color);
          }
        }
      }
      &-equipment {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        &-box {
          position: absolute;
          border: 1px solid #fff;
          &.--success {
            border-color: var(--success-color);
          }
          &.--error {
            border-color: var(--error-color);
          }
        }
      }
    }
  }
</style>
