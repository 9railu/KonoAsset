import { FC, useState } from 'react'
import { useAssetSummaryViewStore } from '@/stores/AssetSummaryViewStore'
import { useAssetFilterStore } from '@/stores/AssetFilterStore'
import { useRefreshFromDisk } from '@/hooks/use-refresh-from-disk'
import { InternalStatusBar } from './internal'

type Props = {
  filterAppliedAssetCount?: number
}

export const StatusBar: FC<Props> = ({ filterAppliedAssetCount }) => {
  const clearFilters = useAssetFilterStore((state) => state.clearFilters)
  const sortedAssetSummaries = useAssetSummaryViewStore(
    (state) => state.sortedAssetSummaries,
  )
  const refreshFromDisk = useRefreshFromDisk()
  const [isRefreshing, setIsRefreshing] = useState(false)

  const handleRefresh = async () => {
    setIsRefreshing(true)
    try {
      await refreshFromDisk()
    } finally {
      setIsRefreshing(false)
    }
  }

  return (
    <InternalStatusBar
      totalAssetCount={sortedAssetSummaries.length}
      filterAppliedAssetCount={filterAppliedAssetCount}
      clearFilters={clearFilters}
      onRefresh={handleRefresh}
      isRefreshing={isRefreshing}
    />
  )
}
