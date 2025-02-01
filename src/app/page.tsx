'use client'

import { useState } from 'react'
import useSWR from 'swr'

interface FactResponse {
  data: string[]
}

const fetcher = async (url: string): Promise<FactResponse> => {
  const res = await fetch(url)
  if (!res.ok) {
    throw new Error('Failed to fetch')
  }
  return res.json()
}

export default function Home() {
  const [language, setLanguage] = useState<string>('eng')
  const { data, error, mutate } = useSWR<FactResponse>(
    `https://meowfacts.herokuapp.com/?count=5&lang=${language}`,
    fetcher,
  )

  const handleLanguageChange = (lang: string) => {
    setLanguage(lang)
    mutate()
  }

  return (
    <>
      <main className='mx-auto max-w-3xl px-6 pb-20 pt-16'>
        <div>
          <h1 className='text-2xl font-bold mb-8'>cat facts</h1>
          <ul className='list-none flex flex-col space-y-8'>
            {error
              ? <li className='font-bold text-red'>failed to load cat facts.</li>
              : !data
              ? <li className='italic'>loading...</li>
              : (
                data.data.map((fact, index) => (
                  <li key={index} className='italic'>
                    {fact}
                  </li>
                ))
              )}
          </ul>
          <div className='flex justify-between mt-8'>
            <button
              onClick={() => mutate()}
              className='underlined'
            >
              refresh
            </button>
            <div className='flex space-x-4'>
              <button
                onClick={() => handleLanguageChange('eng')}
                className={`underlined ${language === 'eng' ? 'font-bold' : 'text-overlay0'}`}
              >
                eng
              </button>
              <button
                onClick={() => handleLanguageChange('rus')}
                className={`underlined ${language === 'rus' ? 'font-bold' : 'text-overlay0'}`}
              >
                rus
              </button>
            </div>
          </div>
        </div>
      </main>
    </>
  )
}
