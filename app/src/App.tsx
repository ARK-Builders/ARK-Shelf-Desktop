import { open } from '@tauri-apps/api/shell';
import {
  AppBar,
  Button,
  Card,
  CardContent,
  Grid,
  List,
  ListItem,
  ListItemText,
  Toolbar,
  Container,
  Typography,
  TextField,
  Pagination,
  IconButton,
  ButtonGroup,
  Tooltip,
} from '@mui/material';
import { useState, useEffect, useCallback } from 'react';
import { invoke, clipboard } from '@tauri-apps/api';
import { ToastContainer, toast, Slide } from 'react-toastify';
import { useForm, SubmitHandler, Controller } from 'react-hook-form';
import 'react-toastify/dist/ReactToastify.css';
import dayjs from 'dayjs';
import {
  ArrowDownward,
  ArrowUpward,
  CalendarMonth,
  FormatListBulleted,
  SortByAlpha,
} from '@mui/icons-material';

interface LinkInfo {
  title: string;
  desc: string;
  url: string;
  name: string;
  created_time: any;
  score: number;
}
interface OpenGraph {
  /// Represents the "og:title" OpenGraph meta tag.
  ///
  /// The title of your object as it should appear within
  /// the graph, e.g., "The Rock".
  title: string;
  /// Represents the "og:description" OpenGraph meta tag
  description: string;
  /// Represents the "og:url" OpenGraph meta tag
  url: string;
  /// Represents the "og:image" OpenGraph meta tag
  image: string;
  /// Represents the "og:type" OpenGraph meta tag
  ///
  /// The type of your object, e.g., "video.movie". Depending on the type
  /// you specify, other properties may also be required.
  object_type: string;
  /// Represents the "og:locale" OpenGraph meta tag
  locale: string;
}
interface LinkScoreMap {
  name: string;
  value: number;
}
interface LinkListItemProps {
  link: LinkInfo;
  index: number;
}
type SortMode = 'normal' | 'date' | 'score';
const Home = () => {
  const [mode, setMode] = useState<SortMode>('normal');

  const [linkInfos, setLinkInfos] = useState<LinkInfo[]>([]);
  const [scores, setScores] = useState<LinkScoreMap[]>([]);

  const {
    register,
    handleSubmit,
    reset,
    setValue,
    getValues,
    control,
    getFieldState,
    formState: { dirtyFields, isDirty },
  } = useForm<LinkInfo>();
  const [page, setPage] = useState(0);
  const itemPerPage = 5;
  const pageCount = Math.ceil(linkInfos.length / itemPerPage);
  console.log(linkInfos);
  console.log('scores:', scores);
  const createLink: SubmitHandler<LinkInfo> = (data) => {
    invoke('create_link', {
      ...data,
    }).then(() => {
      refreshInfo();
      toast('Link created!');
      reset();
      // Forced refreash view to ensure updated list.
    });
  };

  // Refresh all info.
  const refreshInfo = useCallback(async () => {
    const names = (await invoke('read_link_list')) as string[];

    const links = await Promise.all(
      names.map(async (val) => {
        const link = {
          ...(await invoke('read_link', { name: val })),
          name: val,
          score: 0,
        } as LinkInfo;
        return link;
      })
    ).catch((e) => {
      console.error(e);
      throw e;
    });
    const scores = await invoke<LinkScoreMap[]>('get_scores').catch((e) => {
      console.error(e);
      return [] as LinkScoreMap[];
    });
    setScores(scores);
    // Sort and push link infos
    setLinkInfos(
      (links as LinkInfo[])
        // eslint-disable-next-line array-callback-return
        .sort((a, b) => {
          switch (mode) {
            case 'normal':
              return a.title.localeCompare(b.title);
            case 'date':
              return (
                b.created_time.secs_since_epoch -
                a.created_time.secs_since_epoch
              );
            case 'score':
              const item_a =
                scores.find((val) => val.name === a.name)?.value ?? 0;
              const item_b =
                scores.find((val) => val.name === b.name)?.value ?? 0;

              return item_b - item_a;
          }
        })
    );
  }, [mode]);

  useEffect(() => {
    refreshInfo();
  }, [refreshInfo]);

  const LinkList = () => {
    const LinkListItem = ({ link, index }: LinkListItemProps) => {
      const [previewInfo, setPreviewInfo] = useState<OpenGraph>();
      useEffect(() => {
        invoke('generate_link_preview', {
          url: link.url.toString(),
        }).then((val) => setPreviewInfo(val as OpenGraph));
      }, [link.url]);

      return (
        <Tooltip
          arrow
          title={
            <>
              <Typography variant='body2'>
                {previewInfo?.title ??
                  'Preview may not available at the moment'}
              </Typography>
              <img
                loading='lazy'
                alt='preview'
                src={previewInfo?.image}
                width={250}></img>
            </>
          }>
          <ListItem
            dense
            key={index}
            sx={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'flex-start',
            }}>
            <ListItemText
              primary={
                <Typography
                  sx={{
                    maxWidth: '25rem',
                  }}
                  paragraph>
                  {link.title}
                </Typography>
              }
              secondary={dayjs
                .unix(link.created_time.secs_since_epoch)
                .toDate()
                .toLocaleString()}></ListItemText>

            <Grid container>
              <Grid item container m='auto'>
                <Button
                  onClick={() => {
                    clipboard.writeText(link.url);
                  }}>
                  {'COPY'}
                </Button>
                <Button
                  onClick={() => {
                    open(link.url.toString());
                  }}>
                  OPEN
                </Button>
                <Button
                  onClick={() => {
                    invoke('delete_link', {
                      name: link.name,
                    });
                    refreshInfo();
                    toast('Link deleted!');
                  }}
                  color='error'>
                  DELETE
                </Button>

                <IconButton
                  color='primary'
                  disabled={mode !== 'score'}
                  onClick={() => {
                    let arr = Array.from(scores);
                    arr = arr.map((val) => {
                      if (val.name === link.name) {
                        val.value += 1;
                      }
                      return val;
                    });
                    console.log(arr);
                    setScores(arr);
                    invoke('set_scores', {
                      scores: arr,
                    }).then(() => refreshInfo());
                  }}>
                  <ArrowUpward />
                </IconButton>
                <IconButton
                  color='error'
                  disabled={mode !== 'score'}
                  onClick={() => {
                    let arr = Array.from(scores);
                    arr = arr.map((val) => {
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
                      refreshInfo();
                    });
                  }}>
                  <ArrowDownward />
                </IconButton>
                <Typography
                  variant='body1'
                  my={'auto'}
                  sx={{
                    display: mode === 'score' ? 'block' : 'none',
                  }}>
                  Score:
                  {scores.find((val) => val.name === link.name)?.value}
                </Typography>
              </Grid>
            </Grid>
          </ListItem>
        </Tooltip>
      );
    };
    return (
      <List>
        {linkInfos
          .map((val, idx) => {
            return (
              <div id={val.title} key={val.title}>
                <LinkListItem link={val} index={idx} />
              </div>
            );
          })
          .slice(itemPerPage * page, itemPerPage * (page + 1))}
      </List>
    );
  };
  console.log('dirty:', dirtyFields);
  return (
    <>
      <AppBar position='fixed'>
        <Toolbar>
          <Typography
            variant='h6'
            sx={{
              flexGrow: 1,
            }}>
            ARK Shelf
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Container
        sx={{
          mt: 2,
        }}>
        <Grid container spacing={8}>
          <Grid item xs={8}>
            <Card>
              <CardContent>
                <LinkList />

                <Pagination
                  count={pageCount === 0 ? 1 : pageCount}
                  page={page + 1}
                  onChange={(_, page) => {
                    // mui pages are started from 1, against to zero-based index array
                    setPage(page - 1);
                  }}
                  showFirstButton
                  showLastButton
                />
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={4}>
            <Grid item>
              <ButtonGroup variant='outlined'>
                <Tooltip key='alphabet' title={'Sorting By Alphabet'}>
                  <IconButton
                    onClick={() => {
                      setMode('normal');
                    }}>
                    <SortByAlpha />
                  </IconButton>
                </Tooltip>

                <Tooltip key='date' title={'Sorting By Date'}>
                  <IconButton
                    onClick={() => {
                      setMode('date');
                    }}>
                    <CalendarMonth />
                  </IconButton>
                </Tooltip>
                <Tooltip key='score' title={'Sorting By Score'}>
                  <IconButton
                    onClick={() => {
                      setMode('score');
                    }}>
                    <FormatListBulleted />
                  </IconButton>
                </Tooltip>
              </ButtonGroup>
            </Grid>
            <form onSubmit={handleSubmit(createLink)}>
              <Controller
                control={control}
                name='url'
                render={({ field: { value, onChange, onBlur } }) => (
                  <TextField
                    fullWidth
                    label='URL'
                    margin='normal'
                    required={true}
                    onChange={onChange}
                    onBlur={onBlur}
                    value={value ?? ''}
                  />
                )}
              />

              <Controller
                control={control}
                name='title'
                render={({ field: { value, onChange, onBlur } }) => (
                  <TextField
                    fullWidth
                    label='Title'
                    margin='normal'
                    required={true}
                    onChange={onChange}
                    onBlur={onBlur}
                    value={value ?? ''}
                  />
                )}
              />
              <Controller
                control={control}
                name='desc'
                render={({ field: { value, onChange, onBlur } }) => (
                  <TextField
                    fullWidth
                    label='Description (optional)'
                    margin='normal'
                    required={false}
                    onChange={onChange}
                    onBlur={onBlur}
                    value={value ?? ''}
                  />
                )}
              />

              <Button type='submit'>Create</Button>
              <Button
                onClick={() => {
                  if (isDirty && !dirtyFields.name && !dirtyFields.title) {
                    let url = getValues('url');
                    invoke('generate_link_preview', {
                      url: url.toString(),
                    }).then((val) => {
                      let data = val as OpenGraph;
                      setValue('title', data.title, { shouldDirty: true });
                      setValue('desc', data.description, { shouldDirty: true });
                      console.log(dirtyFields);
                    });
                  }
                }}
                color='error'>
                Auto Filled
              </Button>
            </form>
          </Grid>
        </Grid>
      </Container>
      <ToastContainer
        position='bottom-right'
        autoClose={1000}
        hideProgressBar
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        transition={Slide}
        theme='dark'
      />
    </>
  );
};
export default Home;
