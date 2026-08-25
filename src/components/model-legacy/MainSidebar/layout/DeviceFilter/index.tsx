import { useState, useEffect } from 'react'
import { fetchRegisteredDevices, DeviceOption } from './logic'
import { MultiFilterItemSelector } from '@/components/model-legacy/MainSidebar/components/MultiFilterItemSelector'
import { useLocalization } from '@/hooks/use-localization'
import { useAssetFilterStore } from '@/stores/AssetFilterStore'
import { useAssetSummaryViewStore } from '@/stores/AssetSummaryViewStore'
import { useShallow } from 'zustand/react/shallow'

export const DeviceFilter = () => {
  const [devices, setDevices] = useState<DeviceOption[]>([])
  const { t } = useLocalization()

  const { registeredDeviceIds, updateFilter } = useAssetFilterStore(
    useShallow((state) => ({
      registeredDeviceIds: state.filters.registeredDeviceIds,
      updateFilter: state.updateFilter,
    })),
  )

  const sortedAssetSummaries = useAssetSummaryViewStore(
    (state) => state.sortedAssetSummaries,
  )

  useEffect(() => {
    fetchRegisteredDevices().then(setDevices)
  }, [sortedAssetSummaries])

  const nameToId = new Map(devices.map((device) => [device.value, device.id]))
  const idToName = new Map(devices.map((device) => [device.id, device.value]))

  const selectedNames = registeredDeviceIds
    .map((id) => idToName.get(id))
    .filter((name): name is string => name !== undefined)

  return (
    <div className="mt-4">
      <MultiFilterItemSelector
        label={t('mainsidebar:filter:device')}
        placeholder={t('mainsidebar:filter:device:placeholder')}
        candidates={devices}
        value={selectedNames}
        onValueChange={(value) => {
          const ids = value
            .map((name) => nameToId.get(name))
            .filter((id): id is string => id !== undefined)

          updateFilter({ registeredDeviceIds: ids })
        }}
      />
    </div>
  )
}
