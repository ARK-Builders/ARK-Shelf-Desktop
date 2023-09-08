import { ArrowDownward, ArrowUpward } from '@mui/icons-material';
import {
  Button,
  Grid,
  IconButton,
  ListItem,
  ListItemText,
  Tooltip,
  Typography,
} from '@mui/material';
import { clipboard, invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/shell';
import dayjs from 'dayjs';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { LinkInfo, LinkScoreMap, OpenGraph, SortMode } from '../types';

interface LinkListItemProps {
  link: LinkInfo;
  index: number;
  refresh: () => void;
  mode: SortMode;
  scores: LinkScoreMap[];
  setScores: (scores: LinkScoreMap[]) => void;
}

export const LinkListItem = ({
  link,
  index,
  refresh,
  mode,
  scores,
  setScores,
}: LinkListItemProps) => {
  const [previewInfo, setPreviewInfo] = useState<OpenGraph>();

  console.log('Link', { link });
  useEffect(() => {
    invoke('generate_link_preview', {
      url: link.url.toString(),
    }).then(val => setPreviewInfo(val as OpenGraph));
  }, [link.url]);

  return (
    <Tooltip
      arrow
      title={
        <>
          <Typography variant="body2">
            {previewInfo?.title ?? 'Preview may not available at the moment'}
          </Typography>
          {link.desc && <Typography>{link.desc}</Typography>}
          <img loading="lazy" alt="preview" src={previewInfo?.image} width={250}></img>
        </>
      }
    >
      <ListItem
        dense
        key={index}
        sx={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'flex-start',
        }}
      >
        <ListItemText
          primary={
            <Typography
              sx={{
                maxWidth: '25rem',
              }}
              paragraph
            >
              {link.title}
            </Typography>
          }
          secondary={dayjs
            .unix(link.created_time?.secs_since_epoch ?? 0)
            .toDate()
            .toLocaleString()}
        ></ListItemText>

        <Grid container>
          <Grid item container m="auto">
            <Button
              onClick={() => {
                clipboard.writeText(link.url);
              }}
            >
              {'COPY'}
            </Button>
            <Button
              onClick={() => {
                open(link.url.toString());
              }}
            >
              OPEN
            </Button>
            <Button
              onClick={async () => {
                await invoke('delete_link', {
                  name: link.name,
                });
                refresh();
                toast('Link deleted!');
              }}
              color="error"
            >
              DELETE
            </Button>

            <IconButton
              color="primary"
              disabled={mode !== 'score'}
              onClick={() => {
                let arr = Array.from(scores);
                arr = arr.map(val => {
                  if (val.name === link.name) {
                    val.value += 1;
                  }
                  return val;
                });
                setScores(arr);
                invoke('set_scores', {
                  scores: arr,
                }).then(() => refresh());
              }}
            >
              <ArrowUpward />
            </IconButton>
            <IconButton
              color="error"
              disabled={mode !== 'score'}
              onClick={() => {
                let arr = Array.from(scores);
                arr = arr.map(val => {
                  if (val.name === link.name) {
                    val.value -= 1;
                  }
                  return val;
                });
                console.log(arr);
                setScores(arr);
                invoke('set_scores', {
                  scores: arr,
                }).then(() => {
                  refresh();
                });
              }}
            >
              <ArrowDownward />
            </IconButton>
            <Typography
              variant="body1"
              my={'auto'}
              sx={{
                display: mode === 'score' ? 'block' : 'none',
              }}
            >
              Score:
              {scores.find(val => val.name === link.name)?.value}
            </Typography>
          </Grid>
        </Grid>
      </ListItem>
    </Tooltip>
  );
};
