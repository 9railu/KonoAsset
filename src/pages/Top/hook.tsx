import { useState, useEffect, useContext } from 'react'
import { onFileDrop } from './logic'
import { UpdateDialogContext } from '@/components/context/UpdateDialogContext'
import { PreferenceContext } from '@/components/context/PreferenceContext'
import { useDragDropStore } from '@/stores/DragDropStore'
import { DragDropHandler } from '@/stores/DragDropStore/index.types'
import { useRefreshFromDisk } from '@/hooks/use-refresh-from-disk'

type ReturnProps = {
  isDragAndHover: boolean
  showingAssetCount: number
  setShowingAssetCount: (count: number) => void
  addAssetDialogOpen: boolean
  setAddAssetDialogOpen: (open: boolean) => void
  setEditAssetDialogAssetId: (assetId: string | null) => void
  setEditAssetDialogOpen: (open: boolean) => void
  editAssetDialogAssetId: string | null
  editAssetDialogOpen: boolean
}

export const useTopPage = (): ReturnProps => {
  const [addAssetDialogOpen, setAddAssetDialogOpen] = useState(false)

  const [editAssetDialogOpen, setEditAssetDialogOpen] = useState(false)
  const [editAssetDialogAssetId, setEditAssetDialogAssetId] = useState<
    string | null
  >(null)

  const [showingAssetCount, setShowingAssetCount] = useState(0)

  const [isDragAndHover, setDragAndHover] = useState(false)

  const { register } = useDragDropStore()
  const { checkForUpdate } = useContext(UpdateDialogContext)
  const { preference } = useContext(PreferenceContext)
  const refreshFromDisk = useRefreshFromDisk()

  useEffect(() => {
    const handler: DragDropHandler = {
      uniqueId: 'top-page',
      priority: 0,
      fn: async (event) => {
        onFileDrop(event, setDragAndHover)
        return false
      },
    }

    register(handler)
  }, [register])

  useEffect(() => {
    checkForUpdate()
  }, [checkForUpdate])

  // 一覧画面を開いたタイミングで、他デバイスがクラウド同期フォルダに
  // 加えた変更を取り込む。
  useEffect(() => {
    refreshFromDisk()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // 設定された間隔でクラウド同期フォルダ上の変更を定期的に取り込む。
  useEffect(() => {
    const intervalSeconds = preference.autoRefreshIntervalSeconds

    if (!intervalSeconds || intervalSeconds <= 0) {
      return
    }

    const timer = setInterval(() => {
      refreshFromDisk()
    }, intervalSeconds * 1000)

    return () => clearInterval(timer)
  }, [preference.autoRefreshIntervalSeconds, refreshFromDisk])

  return {
    isDragAndHover,
    showingAssetCount,
    setShowingAssetCount,
    addAssetDialogOpen,
    setAddAssetDialogOpen,
    setEditAssetDialogAssetId,
    setEditAssetDialogOpen,
    editAssetDialogAssetId,
    editAssetDialogOpen,
  }
}
