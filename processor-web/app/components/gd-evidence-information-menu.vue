<template>
  <gd-menu label="Evidence Information" :active="active" class="gd-menu">
    <div v-if="evidence" class="gd-menu-card">
      <div class="gd-menu-card-header">
        <div class="gd-menu-card-header-icon">
          <gd-svg name="camera" />
        </div>
        <div class="gd-menu-card-header-information">
          <span class="gd-menu-card-header-information-placeholder gd-body-5">
            Camera
          </span>
          <span class="gd-menu-card-header-information-title gd-headline-5">
            {{ camera ? camera.name : "Unknown Camera" }}
          </span>
        </div>
      </div>
      <div class="gd-menu-card-body">
        <div class="gd-menu-card-body-data">
          <span class="gd-menu-card-body-data-placeholder gd-body-5">Date</span>
          <span class="gd-menu-card-body-data-value gd-headline-6">
            {{ formatTimestamp(evidence.timestamp) }}
          </span>
        </div>
        <div class="gd-menu-card-body-data">
          <span class="gd-menu-card-body-data-placeholder gd-body-5"
            >Violator</span
          >
          <span class="gd-menu-card-body-data-value gd-headline-6">
            {{
              `${evidence.person.filter((a) => a.violation.length > 0).length} person(s)`
            }}
          </span>
        </div>
      </div>
    </div>
    <div v-if="evidence" class="gd-menu-evidence" ref="gdMenuEvidence">
      <div
        class="gd-menu-evidence-wrapper"
        ref="gdMenuEvidenceWrapper"
        :style="{
          width: `${evidenceImageSize.width}px`,
          height: `${evidenceImageSize.height}px`,
          transform: `scale(${evidenceScale}) translate(${evidenceOffset.x}px, ${evidenceOffset.y}px)`,
        }"
      >
        <div class="gd-menu-evidence-wrapper-overlay">
          <div
            v-for="person in evidence.person"
            class="gd-menu-evidence-wrapper-overlay-person"
          >
            <div
              class="gd-menu-evidence-wrapper-overlay-person-box"
              :style="{
                top: `${person.bbox[1] * 100}%`,
                left: `${person.bbox[0] * 100}%`,
                width: `${(person.bbox[2] - person.bbox[0]) * 100}%`,
                height: `${(person.bbox[3] - person.bbox[1]) * 100}%`,
              }"
            ></div>
          </div>
          <div
            v-for="person in evidence.person"
            class="gd-menu-evidence-wrapper-overlay-part"
          >
            <div
              v-for="part in person.part"
              class="gd-menu-evidence-wrapper-overlay-part-box"
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
            class="gd-menu-evidence-wrapper-overlay-equipment"
          >
            <div
              v-for="equipment in person.equipment"
              class="gd-menu-evidence-wrapper-overlay-equipment-box"
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
    </div>
    <div class="gd-menu-control">
      <gd-button type="tertiary" :icon="'arrow-left'" @click="personPrev" />
      <div class="gd-menu-control-information">
        <span class="gd-menu-control-information-value gd-headline-5">{{
          `Person ${evidencePersonIndex + 1} / ${evidence?.person.length || 0}`
        }}</span>
        <div class="gd-menu-control-information-placeholder gd-body-5">
          {{
            `${evidence?.person[evidencePersonIndex]?.violation.length || 0} violation(s)`
          }}
        </div>
      </div>
      <gd-button type="tertiary" :icon="'arrow-right'" @click="personNext" />
    </div>
    <div
      v-for="violation in evidence?.person[evidencePersonIndex]?.violation ||
      []"
      class="gd-menu-violation"
    >
      <div class="gd-menu-violation-header">
        <div class="gd-menu-violation-header-icon">
          <gd-svg :name="violationIcon(violation)" />
        </div>
        <span class="gd-menu-violation-header-title gd-headline-5">
          {{ violationTitle(violation) }}
        </span>
      </div>
      <span class="gd-menu-violation-body gd-body-5">
        {{ violationDescription(violation) }}
      </span>
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
  import type {
    EvidencePersonEquipmentLabel,
    EvidencePersonPartLabel,
    EvidencePersonViolation,
  } from "~/types/evidence";

  const emits = defineEmits(["shake"]);
  const props = defineProps<{
    active: boolean;
    camera_id: string;
    evidence_id: string;
  }>();

  const {
    public: { processor: api },
  } = useRuntimeConfig();
  const { cameras } = useCamera();
  const { evidences } = useEvidence();

  const evidence = computed(() => {
    return evidences.value.find((e) => e.id === props.evidence_id) || null;
  });
  const camera = computed(() => {
    return cameras.value.find((c) => c.id === props.camera_id) || null;
  });

  const gdMenuEvidenceWrapper = ref<HTMLDivElement>();
  const gdMenuEvidence = ref<HTMLDivElement>();

  const evidencePersonIndex = ref(-1);
  const evidenceImageSize = ref<{ width: number; height: number }>({
    width: 0,
    height: 0,
  });
  const evidenceScale = ref(1);
  const evidenceOffset = ref<{ x: number; y: number }>({ x: 0, y: 0 });

  function personNext() {
    if (!evidence.value) return;
    evidencePersonIndex.value++;
    if (evidencePersonIndex.value >= evidence.value.person.length) {
      evidencePersonIndex.value = 0;
    }
  }
  function personPrev() {
    if (!evidence.value) return;
    evidencePersonIndex.value--;
    if (evidencePersonIndex.value < 0) {
      evidencePersonIndex.value = evidence.value.person.length - 1;
    }
  }
  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleString();
  }
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
  function violationIcon(violation: EvidencePersonViolation): string {
    if (violation.includes("hardhat")) return "hardhat";
    if (violation.includes("gloves")) return "gloves";
    if (violation.includes("shoes")) return "shoes";
    if (violation.includes("facemask")) return "facemask";
    if (violation.includes("earmuffs")) return "earmuffs";
    if (violation.includes("safetyvest")) return "safetyvest";
    return "alert";
  }
  function violationTitle(violation: EvidencePersonViolation): string {
    if (violation === "improperly_worn_hardhat")
      return "Hardhat Improperly Worn";
    if (violation === "improperly_worn_gloves") return "Gloves Improperly Worn";
    if (violation === "improperly_worn_shoes") return "Shoes Improperly Worn";
    if (violation === "improperly_worn_facemask")
      return "Facemask Improperly Worn";
    if (violation === "improperly_worn_earmuffs")
      return "Earmuffs Improperly Worn";
    if (violation === "missing_hardhat") return "Missing Hardhat";
    if (violation === "missing_gloves") return "Missing Gloves";
    if (violation === "missing_shoes") return "Missing Shoes";
    if (violation === "missing_facemask") return "Missing Facemask";
    if (violation === "missing_earmuffs") return "Missing Earmuffs";
    if (violation === "missing_safetyvest") return "Missing Safety Vest";
    return "Violation";
  }
  function violationDescription(violation: EvidencePersonViolation): string {
    if (violation === "improperly_worn_hardhat")
      return "Both head and hardhat is detected, but hardhat isn't placed on head properly.";
    if (violation === "improperly_worn_gloves")
      return "Both hand and gloves is detected, but gloves isn't covering the hand properly.";
    if (violation === "improperly_worn_shoes")
      return "Both foot and shoes is detected, but shoes isn't covering the foot properly.";
    if (violation === "improperly_worn_facemask")
      return "Both face and facemask is detected, but facemask isn't covering the face properly.";
    if (violation === "improperly_worn_earmuffs")
      return "Both ear and earmuffs is detected, but earmuffs isn't covering the ear properly.";
    if (violation === "missing_hardhat")
      return "Head is detected, but hardhat isn't detected.";
    if (violation === "missing_gloves")
      return "Hand is detected, but gloves isn't detected.";
    if (violation === "missing_shoes")
      return "Foot is detected, but shoes isn't detected.";
    if (violation === "missing_facemask")
      return "Face is detected, but facemask isn't detected.";
    if (violation === "missing_earmuffs")
      return "Ear is detected, but earmuffs isn't detected.";
    if (violation === "missing_safetyvest")
      return "Safety vest isn't detected on the person.";
    return "Violation";
  }

  watch(
    () => evidencePersonIndex.value,
    (val) => {
      const person = evidence.value?.person[val];
      if (!person) return;

      const viewSize = gdMenuEvidence.value?.getBoundingClientRect().width || 0;
      const margin = 0.12;

      const w = person.bbox[2] - person.bbox[0];
      const h = person.bbox[3] - person.bbox[1];

      let pw = 0,
        ph = 0,
        px = 0,
        py = 0,
        scale = 1;

      if (w > h) {
        pw = w * (1 + margin * 2);
        scale =
          (1 / pw) *
          (viewSize /
            Math.min(
              evidenceImageSize.value.width,
              evidenceImageSize.value.height,
            ));

        px = (person.bbox[0] - (pw - w) / 2) * evidenceImageSize.value.width;
        py =
          (person.bbox[1] +
            h / 2 -
            (viewSize * 0.5) / evidenceImageSize.value.height / scale) *
          evidenceImageSize.value.height;
      } else if (w < h) {
        ph = h * (1 + margin * 2);
        scale =
          (1 / ph) *
          (viewSize /
            Math.min(
              evidenceImageSize.value.width,
              evidenceImageSize.value.height,
            ));

        px =
          (person.bbox[0] +
            w / 2 -
            (viewSize * 0.5) / evidenceImageSize.value.width / scale) *
          evidenceImageSize.value.width;
        py = (person.bbox[1] - (ph - h) / 2) * evidenceImageSize.value.height;
      } else {
        pw = w * (1 + margin * 2);
        ph = h * (1 + margin * 2);
        scale =
          (1 / pw) *
          (viewSize /
            Math.min(
              evidenceImageSize.value.width,
              evidenceImageSize.value.height,
            ));

        px = (person.bbox[0] - (pw - w) / 2) * evidenceImageSize.value.width;
        py = (person.bbox[1] - (ph - h) / 2) * evidenceImageSize.value.height;
      }

      evidenceScale.value = scale;
      evidenceOffset.value = { x: -px, y: -py };
    },
  );

  onMounted(async () => {
    // Load image
    if (evidence.value) {
      const img = new Image();
      img.src = `${api}/evidence/${evidence.value.id}`;
      img.className = "gd-menu-evidence-wrapper-image";
      img.style.zIndex = "0";
      await img.decode();
      if (gdMenuEvidenceWrapper.value) {
        gdMenuEvidenceWrapper.value.appendChild(img);
        evidenceImageSize.value = {
          width: img.naturalWidth,
          height: img.naturalHeight,
        };
        evidencePersonIndex.value = 0;
      }
    }
  });
</script>

<style lang="scss" scoped>
  .gd-menu {
    position: relative;
    display: flex;
    flex-direction: column;
    overflow-y: auto;

    &-card {
      position: relative;
      width: 100%;
      padding: 0.75rem;
      border-radius: 0.75rem;
      border: var(--border);
      box-sizing: border-box;
      background: var(--background-depth-one-color);
      margin-bottom: 1rem;
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &-header {
        position: relative;
        width: 100%;
        display: flex;
        gap: 0.5rem;
        &-icon {
          position: relative;
          width: 2rem;
          height: 2rem;
          background-color: var(--background-depth-two-color);
          padding: 0 0.5rem;
          border-radius: 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: center;
          align-items: center;
        }
        &-information {
          position: relative;
          display: flex;
          flex-direction: column;
          &-placeholder {
            color: var(--font-secondary-color);
          }
        }
      }

      &-body {
        position: relative;
        width: 100%;
        display: flex;
        &-data {
          position: relative;
          width: 100%;
          display: flex;
          flex-direction: column;
          gap: 0.125rem;
          &-placeholder {
            color: var(--font-secondary-color);
          }
        }
      }
    }

    &-evidence {
      position: relative;
      width: 100%;
      aspect-ratio: 1 / 1;
      background-color: var(--background-depth-one-color);
      border: var(--border);
      border-radius: 0.75rem;
      display: flex;
      overflow: hidden;

      &-wrapper {
        transform-origin: top left;
        * {
          pointer-events: none;
        }

        &-overlay {
          z-index: 1;
          position: absolute;
          width: 100%;
          height: 100%;
          &-person {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            &-box {
              position: absolute;
              border: 2px solid #fff;
              box-sizing: border-box;
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
              border: 2px solid #fff;
              box-sizing: border-box;
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
              border: 2px solid #fff;
              box-sizing: border-box;
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
    }

    &-control {
      position: relative;
      width: 100%;
      margin: 0.75rem 0;
      display: flex;
      justify-content: space-between;
      align-items: center;
      &-information {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        &-value {
          position: relative;
        }
        &-placeholder {
          position: relative;
          color: var(--font-secondary-color);
        }
      }
    }

    &-violation {
      position: relative;
      width: 100%;
      padding: 0.75rem;
      border-radius: 0.75rem;
      border: var(--border);
      box-sizing: border-box;
      background: var(--background-depth-one-color);
      margin-bottom: 1rem;
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &-header {
        position: relative;
        width: 100%;
        display: flex;
        gap: 0.5rem;
        &-icon {
          position: relative;
          width: 2rem;
          height: 2rem;
          background-color: var(--background-depth-two-color);
          padding: 0 0.5rem;
          border-radius: 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: center;
          align-items: center;
        }
        &-title {
          position: relative;
        }
      }

      &-body {
        position: relative;
        width: 100%;
        padding: 0.5rem;
        border-radius: 0.5rem;
        box-sizing: border-box;
        background-color: var(--background-depth-two-color);
        color: var(--font-secondary-color);
      }
    }
  }
</style>
