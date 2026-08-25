import { commands } from '@/lib/bindings'
import { useAssetFilterStore } from '@/stores/AssetFilterStore'
import { useAssetSummaryViewStore } from '@/stores/AssetSummaryViewStore'
import { useCallback } from 'react'

// クラウド同期フォルダ上で他デバイスが追加・変更したアセットを取り込むため、
// ディスク上の metadata ファイルを読み直した上で、一覧・フィルタの表示を更新する。
export const useRefreshFromDisk = () => {
  const refreshAssetSummaries = useAssetSummaryViewStore(
    (state) => state.refreshAssetSummaries,
  )
  const refreshFilteredIds = useAssetFilterStore(
    (state) => state.refreshFilteredIds,
  )

  return useCallback(async () => {
    const result = await commands.refreshAssetsFromDisk()

    if (result.status === 'error') {
      console.error(result.error)
      return
    }

    await Promise.all([refreshAssetSummaries(), refreshFilteredIds()])
  }, [refreshAssetSummaries, refreshFilteredIds])
}
